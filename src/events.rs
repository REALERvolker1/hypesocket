use crate::abstractions::{BufRead, BufReader, UnixStream};
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
    crate::abstractions::tuple_vec_impls!(Vec<u8>);
}

macro_rules! event_socket_impl {
    ($(- $async:ident, $await:ident )? ) => {
        crate::abstractions::socket_impls!(PATH_NAME $(, $async, $await)?);

        /// Create a hew connection from a custom path.
        ///
        /// Use this if the Hyprland socket is not in the default location, or if the default location has changed.
        #[inline]
        pub $( $async )? fn new_from_path(path: &::std::path::Path) -> ::std::io::Result<Self> {
            Ok(Self {
                sock: BufReader::new(UnixStream::connect(path)$(.$await)??),
                known_size_clonable_buffer: Vec::new(),
            })
        }

        /// Read the next event data
        #[cfg_attr(feature = "tracing", tracing::instrument(level = "trace"))]
        pub $( $async )? fn next_event(&mut self) -> std::io::Result<RawHyprlandEventData> {
            self.sock
                .read_until(b'\n', &mut self.known_size_clonable_buffer)$(.$await)??;

            self.known_size_clonable_buffer.shrink_to_fit();
            let output = RawHyprlandEventData(self.known_size_clonable_buffer.clone());

            self.known_size_clonable_buffer.clear();

            Ok(output)
        }
    };
}

/// A socket meant for reading Hyprland events
#[derive(Debug)]
pub struct HyprlandEventSocket {
    sock: BufReader<UnixStream>,
    known_size_clonable_buffer: Vec<u8>,
}
impl HyprlandEventSocket {
    #[cfg(any(feature = "tokio", feature = "async-lite"))]
    event_socket_impl!(- async, await);
    #[cfg(all(not(feature = "tokio"), not(feature = "async-lite")))]
    event_socket_impl!();
}
