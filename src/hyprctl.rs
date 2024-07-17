use crate::abstractions::{ReadExt, UnixStream, WriteExt};

const PATH_NAME: &str = ".socket.sock";

/// A flag that can be passed to hyprctl
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CtlFlag {
    Json,
    Custom(&'static str),
    #[default]
    None,
}
impl CtlFlag {
    /// Get the bytes representation of the flag. Used internally in [`Hyprctl`]
    pub const fn as_bytes(&self) -> &'static [u8] {
        match self {
            Self::Json => &[b'j'],
            Self::Custom(s) => s.as_bytes(),
            Self::None => &[],
        }
    }
}

/// An owned byte buffer that can be sent to hyprctl verbatim.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Hyprctl(Vec<u8>);
impl Hyprctl {
    /// Create a hew Hyprctl-ready buffer
    #[inline]
    pub fn new(flags: Option<&[CtlFlag]>, command: &[&str]) -> Self {
        Self::new_from_buffer(flags, command, Vec::new())
    }

    /// Create a new Hyprctl using a currently allocated buffer
    pub fn new_from_buffer(
        flags: Option<&[CtlFlag]>,
        command: &[&str],
        mut buffer: Vec<u8>,
    ) -> Self {
        buffer.clear();
        if let Some(flags) = flags {
            flags.into_iter().for_each(|f| {
                buffer.extend_from_slice(f.as_bytes());
            });
        }

        buffer.push(b'/');

        command.into_iter().for_each(|a| {
            buffer.extend_from_slice(a.as_bytes());
            buffer.push(b' ');
        });

        let last_char_idx = buffer.len() - 1;
        buffer[last_char_idx] = b'\n';

        Self(buffer)
    }

    crate::abstractions::tuple_vec_impls!(Vec<u8>);
}

macro_rules! hyprctl_socket_impl {
    ($( $async:tt, $await:tt )? ) => {
        crate::abstractions::socket_impls!(PATH_NAME $($async, $await)?);
        /// Run a pre-allocated, pre-formatted hyprctl command.
        #[cfg_attr(feature = "tracing", tracing::instrument(level = "trace"))]
        pub $($async)? fn run_hyprctl(&mut self, command: &Hyprctl) -> std::io::Result<Vec<u8>> {
            self.sock.write_all(command.bytes())$(.$await)??;
            let mut output_buffer = Vec::new();
            self.sock.read_to_end(&mut output_buffer)$(.$await)??;

            Ok(output_buffer)
        }

        pub $($async)? fn dispatch_exec(socket: &mut HyprctlSocket, shell_command: &str) -> std::io::Result<()> {
            socket.run_hyprctl(&Hyprctl::new(None, &["dispatch", "--", "exec", shell_command]))?;
            Ok(())
        }
    };
}

#[derive(Debug)]
pub struct HyprctlSocket {
    sock: UnixStream,
}
impl HyprctlSocket {
    #[cfg(any(feature = "tokio", feature = "async-lite"))]
    hyprctl_socket_impl!(async, await);
    #[cfg(all(not(feature = "tokio"), not(feature = "async-lite")))]
    hyprctl_socket_impl!();
}
