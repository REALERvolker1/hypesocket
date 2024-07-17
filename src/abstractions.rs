#[cfg(feature = "tokio")]
pub use tokio::{
    io::{AsyncReadExt as ReadExt, AsyncWriteExt as WriteExt},
    net::UnixStream,
};

#[cfg(all(not(feature = "tokio"), feature = "async-lite"))]
pub use async_net::unix::UnixStream;
#[cfg(all(not(feature = "tokio"), feature = "async-lite"))]
pub use futures_lite::{AsyncReadExt as ReadExt, AsyncWriteExt as WriteExt};

#[cfg(all(not(feature = "tokio"), not(feature = "async-lite")))]
pub use std::{io::Read as ReadExt, io::Write as WriteExt, os::unix::net::UnixStream};

pub type IoResult<T> = ::std::result::Result<T, ::std::io::Error>;

macro_rules! socket_impls {
    ($socket_name:expr $(, $async:tt, $await:tt )? ) => {
        /// Create a hew connection from a custom path.
        ///
        /// Use this if the Hyprland socket is not in the default location, or if the default location has changed.
        #[inline]
        pub $( $async )? fn new_from_path(path: &::std::path::Path) -> ::std::io::Result<Self> {
            Ok(Self {
                sock: UnixStream::connect(path)$(.$await)??,
            })
        }

        /// Create a hew connection from the environment variables.
        ///
        /// If this function does not work, first ensure you are using Hyprland. Then, ensure that the `new_from_path` function works.
        #[inline]
        pub $( $async )? fn new_from_env() -> ::std::io::Result<Self> {
            let my_path = $crate::abstractions::env_get_inner($socket_name)?;
            let me = Self::new_from_path(&my_path)$(.$await)??;
            Ok(me)
        }
    };
}

macro_rules! tuple_vec_impls {
    () => {
        /// Try to convert the buffer into a string
        #[inline]
        pub fn try_as_str(&self) -> Result<&str, std::str::Utf8Error> {
            std::str::from_utf8(&self.0)
        }

        /// Create a new instance from a currently allocated buffer, without checking if it is valid
        #[inline]
        pub fn new_from_raw(buffer: Vec<u8>) -> Self {
            Self(buffer)
        }

        /// Get the length of the inner buffer
        #[inline]
        pub fn len(&self) -> usize {
            self.0.len()
        }

        /// Get a reference to the buffer's raw data
        #[inline]
        pub fn bytes(&self) -> &[u8] {
            &self.0
        }

        /// Deconstruct/destroy this, returning the inner buffer
        #[inline]
        pub fn into_inner(self) -> Vec<u8> {
            self.0
        }
    };
}

pub(crate) use {socket_impls, tuple_vec_impls};

/// A helper function for getting the paths to the hyprland sockets.
pub(crate) fn env_get_inner(socket_name: &str) -> std::io::Result<std::path::PathBuf> {
    let hyprland_instance = std::env::var_os("HYPRLAND_INSTANCE_SIGNATURE").ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            "HYPRLAND_INSTANCE_SIGNATURE is not set! (Are you using Hyprland?)",
        )
    })?;

    let mut path =
        std::path::PathBuf::from(std::env::var_os("XDG_RUNTIME_DIR").ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                "XDG_RUNTIME_DIR is not set! (Are you using Linux?)",
            )
        })?);

    path.push("hypr");
    path.push(&hyprland_instance);
    path.push(socket_name);

    Ok(path)
}
