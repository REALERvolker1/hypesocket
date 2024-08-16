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

        // remove last space
        buffer.pop();
        // let last_char_idx = buffer.len() - 1;
        // buffer[last_char_idx] = b'\n';

        Self(buffer)
    }

    crate::abstractions::tuple_vec_impls!(Vec<u8>);
}

macro_rules! hyprctl_socket_impl {
    ($(- $async:ident, $await:ident )? ) => {
        crate::abstractions::socket_impls!(PATH_NAME $(, $async, $await)?);

        /// Create a hew connection from a custom path.
        ///
        /// Use this if the Hyprland socket is not in the default location, or if the default location has changed.
        #[inline]
        pub $( $async )? fn new_from_path(path: &::std::path::Path) -> ::std::io::Result<Self> {
            Ok(Self {
                sock: UnixStream::connect(path)$(.$await)??,
            })
        }

        /// Run a pre-allocated, pre-formatted hyprctl command.
        #[inline]
        pub $($async)? fn run_hyprctl(&mut self, command: &Hyprctl) -> std::io::Result<Vec<u8>> {
            self.send_bytes(command.bytes())$(.$await)?
        }

        /// Send raw data to hyprctl
        #[cfg_attr(feature = "tracing", tracing::instrument(level = "trace"))]
        pub $($async)? fn send_bytes(&mut self, bytes: &[u8]) -> std::io::Result<Vec<u8>> {
            self.sock.write_all(bytes)$(.$await)??;
            let mut output_buffer = Vec::new();
            self.sock.read_to_end(&mut output_buffer)$(.$await)??;

            Ok(output_buffer)
        }

        /// A shortcut helper function for running `hyprctl dispatch -- exec <command --args>`
        #[inline]
        pub $($async)? fn dispatch_exec(&mut self, shell_command: &str) -> std::io::Result<()> {
            self.run_hyprctl(&Hyprctl::new(None, &["dispatch", "--", "exec", shell_command]))$(.$await)??;
            Ok(())
        }
        #[cfg(feature = "json_commands")]
        /// A shortcut helper function for running `hyprctl dispatch -- exec <command --args>`
        #[inline]
        pub $($async)? fn get_monitors(&mut self) -> std::io::Result<Vec<$crate::json_commands::Monitor>> {
            let monitors = self.send_bytes(b"j/monitors\n")$(.$await)??;
            let monitors = serde_json::from_slice(&monitors)?;
            Ok(monitors)
        }
    };
}

#[derive(Debug)]
pub struct HyprctlSocket {
    sock: UnixStream,
}
impl HyprctlSocket {
    #[cfg(any(feature = "tokio", feature = "async-lite"))]
    hyprctl_socket_impl!(- async, await);
    #[cfg(all(not(feature = "tokio"), not(feature = "async-lite")))]
    hyprctl_socket_impl!();
}
