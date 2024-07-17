use std::collections::HashMap;

use serde::{Deserialize, Serialize};

pub type ScreenCoordinateInt = i32;
pub type ScreenSizeInt = u32;
pub type MonitorIdInt = i32;
pub type WorkspaceId = i32;
/// For your system, consult `/proc/sys/kernel/pid_max`
pub type PidInt = isize;
pub type MonitorName = String;

/// This looks like an integer to my eyes. hyprctl -j emits it as a String. It ix 0x<number> in hexadecimal
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct HexIntAddress(pub String);
impl HexIntAddress {
    /// Try to convert this address to a usize for easier use
    pub fn convert_to_usize(&self) -> Option<usize> {
        let my_bytes = self.0.as_bytes();

        if my_bytes.len() < 2 || my_bytes[0] != b'0' || my_bytes[1] != b'x' {
            return None;
        }

        let number_bytes = &my_bytes[2..];
        let (result, len) = atoi::FromRadix16::from_radix_16(number_bytes);
        if len != number_bytes.len() {
            return None;
        }

        Some(result)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MinimalWorkspace {
    pub id: WorkspaceId,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Monitor {
    pub id: MonitorIdInt,
    pub name: MonitorName,
    pub description: String,
    pub make: String,
    pub model: String,
    pub serial: String,
    pub width: ScreenSizeInt,
    pub height: ScreenSizeInt,
    pub refresh_rate: f32,
    pub x: ScreenCoordinateInt,
    pub y: ScreenCoordinateInt,
    pub active_workspace: MinimalWorkspace,
    pub special_workspace: Option<MinimalWorkspace>,
    pub reserved: [ScreenSizeInt; 4],
    pub scale: f32,
    pub transform: ScreenCoordinateInt,
    pub focused: bool,
    pub dpms_status: bool,
    pub vrr: bool,
    pub actively_tearing: bool,
    pub disabled: bool,
    pub current_format: String,
    /// This should be in the format of `["1920x1080@60.00Hz","1680x1050@59.88Hz","1280x1024@75.03Hz",...]`
    pub available_modes: Vec<String>,
}
impl Monitor {
    /// Parse json for this monitor.
    #[inline]
    pub fn from_json_bytes(json: &[u8]) -> Result<Vec<Self>, serde_json::Error> {
        serde_json::from_slice(json)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Client {
    /// This is output as a string but it is actually a hexadecimal integer like "0x58481cd30ae0"
    pub address: HexIntAddress,
    pub mapped: bool,
    pub hidden: bool,
    pub at: [ScreenCoordinateInt; 2],
    pub size: [ScreenSizeInt; 2],
    pub workspace: MinimalWorkspace,
    pub floating: bool,
    pub pseudo: bool,
    pub monitor: MonitorIdInt,
    pub class: String,
    pub title: String,
    #[serde(rename = "initialClass")]
    pub initial_class: String,
    #[serde(rename = "initialTitle")]
    pub initial_title: String,
    pub pid: PidInt,
    pub xwayland: bool,
    pub pinned: bool,
    pub fullscreen: bool,
    #[serde(rename = "fullscreenMode")]
    pub fullscreen_mode: u8,
    #[serde(rename = "fakeFullscreen")]
    pub fake_fullscreen: bool,
    pub grouped: Vec<HexIntAddress>,
    /// Not supported by this wrapper
    pub tags: Vec<String>,
    /// TODO: Find out if this is a window address
    pub swallowing: String,
    #[serde(rename = "focusHistoryID")]
    pub focus_history_id: WorkspaceId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorkspaceFull {
    pub id: WorkspaceId,
    pub name: String,
    pub monitor: MonitorName,
    #[serde(rename = "monitorID")]
    pub monitor_id: MonitorIdInt,
    pub windows: usize,
    pub hasfullscreen: bool,
    pub lastwindow: HexIntAddress,
    pub lastwindowtitle: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Keyboard {
    pub address: HexIntAddress,
    pub name: String,
    pub rules: String,
    pub model: String,
    pub layout: String,
    pub variant: String,
    pub options: String,
    pub active_keymap: String,
    pub main: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Switch {
    pub address: HexIntAddress,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mouse {
    pub address: HexIntAddress,
    pub name: String,
    #[serde(rename = "defaultSpeed")]
    pub default_speed: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Version {
    pub branch: String,
    pub commit: String,
    pub dirty: bool,
    pub commit_message: String,
    pub commmit_date: String,
    pub tag: String,
    /// TODO: This looks like an int passed as a string
    pub commits: String,
    /// TODO: My hyprctl shows this as empty
    pub flags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayerList {
    pub levels: HashMap<String, Vec<Layer>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Layer {
    pub address: HexIntAddress,
    pub x: ScreenCoordinateInt,
    pub y: ScreenCoordinateInt,
    #[serde(rename = "w")]
    pub width: ScreenSizeInt,
    #[serde(rename = "h")]
    pub height: ScreenSizeInt,
    pub namespace: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CursorPos {
    pub x: ScreenCoordinateInt,
    pub y: ScreenCoordinateInt,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HyprlandInstance {
    pub instance: String,
    pub time: u64,
    pub pid: PidInt,
}
