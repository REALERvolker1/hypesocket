use crate::abstractions::{IoResult, ReadExt, UnixStream};
const PATH_NAME: &str = ".socket2.sock";

/// Raw, unparsed lines read from the Hyprland event socket
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RawHyprlandEventData(Vec<u8>);
impl RawHyprlandEventData {
    /// This tries to parse the event data. It allocates a few new Strings.
    ///
    /// The current implementation follows the hyprctl wiki format of `EVENT>>DATA`,
    /// but this is all subject to change. If this does not work for you, please open an issue or parse it yourself.
    #[inline]
    pub fn try_parse_string(&self) -> Option<(String, String)> {
        let mut name_buffer = Vec::new();
        let mut data_buffer = Vec::new();

        self.parse_into_buffers(&mut name_buffer, &mut data_buffer)?;

        let name = String::from_utf8(name_buffer).ok()?;
        let data = String::from_utf8(data_buffer).ok()?;
        Some((name, data))
    }

    /// This tries to parse the event data into pre-allocated buffers.
    /// - Returns `Some(())` if it was successful.
    /// - Returns `None` if it was unsuccessful and the buffers were not mutated.
    pub fn parse_into_buffers(
        &self,
        name_buffer: &mut Vec<u8>,
        data_buffer: &mut Vec<u8>,
    ) -> Option<()> {
        for idx in 0..self.0.len() {
            if self.0[idx] == b'>' {
                let next_idx = idx.checked_add(1)?;
                if self.0[next_idx] == b'>' {
                    let data_start_idx = next_idx.checked_add(1)?;
                    name_buffer.clear();
                    data_buffer.clear();

                    name_buffer.extend_from_slice(&self.0[0..idx]);
                    data_buffer.extend_from_slice(&self.0[data_start_idx..]);

                    return Some(());
                }
            }
        }

        None
    }
    crate::abstractions::tuple_vec_impls!();
}

/// A socket meant for reading Hyprland events
#[derive(Debug)]
pub struct HyprlandEventSocket {
    sock: UnixStream,
}
impl HyprlandEventSocket {
    #[cfg(all(not(feature = "tokio"), not(feature = "async-lite")))]
    crate::abstractions::socket_impls!(PATH_NAME);
    #[cfg(any(feature = "tokio", feature = "async-lite"))]
    crate::abstractions::socket_impls!(PATH_NAME, async, await);

    /// Read potentially many events at a time.
    ///
    /// This returns once there is no more data to read.
    #[cfg(any(feature = "tokio", feature = "async-lite"))]
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "trace"))]
    pub async fn read_events(&mut self) -> IoResult<Vec<RawHyprlandEventData>> {
        let mut event_vec = Vec::new();
        self.sock.read_to_end(&mut event_vec).await?;

        Ok(event_vec
            .split(|b| *b == b'\n')
            .map(|v| RawHyprlandEventData(v.to_vec()))
            .collect())
    }
    #[cfg(all(not(feature = "tokio"), not(feature = "async-lite")))]
    #[cfg_attr(feature = "tracing", tracing::instrument(level = "trace"))]
    pub fn read_events(&mut self) -> IoResult<Vec<RawHyprlandEventData>> {
        let mut event_vec = Vec::new();
        self.sock.read_to_end(&mut event_vec)?;

        Ok(event_vec
            .split(|b| *b == b'\n')
            .map(|v| RawHyprlandEventData(v.to_vec()))
            .collect())
    }
}
