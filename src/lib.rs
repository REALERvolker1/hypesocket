//! A simple wrapper around hyprland's socket IPC.
//!
//! This library is very low-level. You will need to refer to the [official documentation](https://wiki.hyprland.org/IPC/).

mod abstractions;
pub mod events;
pub mod hyprctl;

#[cfg(all(test, not(feature = "tokio"), not(feature = "async-lite")))]
mod tests {
    use crate::hyprctl::{Hyprctl, HyprctlSocket};
    #[test]
    fn exec_kitty() {
        let mut connection = HyprctlSocket::new_from_env().expect("Failed to connect to hyprland!");

        let my_command = Hyprctl::new(None, &["dispatch", "exec", "kitty"]);
        let command_string = my_command.try_as_str().unwrap();
        assert_eq!(command_string, "/dispatch exec kitty\n");

        let output = connection
            .run_hyprctl(&my_command)
            .expect("Failed to run command!");

        let output_str = String::from_utf8_lossy(&output);
        assert_eq!(output_str, "ok");
    }
}
