//! A simple wrapper around hyprland's socket IPC.
//!
//! This library is very low-level. You will need to refer to the [official documentation](https://wiki.hyprland.org/IPC/).

mod abstractions;
pub mod events;
pub mod hyprctl;

#[cfg(feature = "json_commands")]
pub mod json_commands;

#[cfg(all(test, not(feature = "tokio"), not(feature = "async-lite")))]
mod tests {
    use crate::{
        events::HyprlandEventSocket,
        hyprctl::{CtlFlag, Hyprctl, HyprctlSocket},
    };
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

    #[test]
    fn event_listener() {
        let mut conn = HyprlandEventSocket::new_from_env().expect("Failed to connect to hyprland!");
        for _ in 0..5 {
            let event = conn.next_event().expect("Failed to read events!");

            let (event, data) = event.try_parse_string().expect("Failed to parse event!");
            println!("[{}] {}", event, data);
        }
    }

    #[cfg(feature = "json_commands")]
    #[test]
    fn json_monitors() {
        let mut connection = HyprctlSocket::new_from_env().expect("Failed to connect to hyprland!");
        let monitors = connection.get_monitors().expect("Failed to get monitors!");
    }
}
