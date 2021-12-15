extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate serde_repr;

use serde::Deserializer;
use serde_repr::Deserialize_repr;

// Response structs
// Run Command
#[derive(Deserialize, Debug, PartialEq)]
pub struct CommandResult {
    pub success: bool,
    pub error: Option<String>
}

// Get Workspaces
#[derive(Deserialize, Debug, PartialEq)]
pub struct Workspace {
    pub num: i64,
    pub name: String,
    pub visible: bool,
    pub focused: bool,
    pub urgent: bool,
    pub output: String,
    pub rect: Rectangle
}
#[derive(Deserialize, Debug, PartialEq)]
pub struct Rectangle {
    pub x: u64,
    pub y: u64,
    pub width: u64,
    pub height: u64
}

// Get Output
#[derive(Deserialize, Debug, PartialEq)]
pub struct Output {
    pub name: String,
    pub make: String,
    pub model: String,
    pub serial: String,
    pub active: bool,
    pub dpms: bool,
    pub primary: bool,
    pub scale: f64,
    pub subpixel_hinting: SubpixelHinting,
    pub transform: OutputTransform,
    pub current_workspace: String,
    pub modes: Vec<OutputMode>,
    pub current_mode: OutputMode,
    pub rect: Rectangle
}
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum SubpixelHinting { RGB, BGR, VRGB, VBGR, None, Unknown }
#[derive(Deserialize, Debug, PartialEq)]
pub enum OutputTransform {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "90")]
    Right,
    #[serde(rename = "180")]
    Half,
    #[serde(rename = "270")]
    Left,
    #[serde(rename = "flipped-90")]
    FlippedRight,
    #[serde(rename = "flipped-180")]
    FlippedHalf,
    #[serde(rename = "flipped-270")]
    FlippedLeft
}
#[derive(Deserialize, Debug, PartialEq)]
pub struct OutputMode {
    pub width: u64,
    pub height: u64,
    pub refresh: u64
}

// Get Tree
#[derive(Deserialize, Debug, PartialEq)]
pub struct Node {
    pub id: u64,
    pub name: String,
    #[serde(rename = "type")]
    pub node_type: NodeType,
    pub border: NodeBorder,
    pub current_border_width: u8,
    pub layout: NodeLayout,
    pub orientation: NodeOrientation,
    //percent: Option<f64>,
    pub rect: Rectangle,
    pub window_rect: Rectangle,
    pub deco_rect: Rectangle,
    pub geometry: Rectangle,
    pub urgent: bool,
    pub sticky: bool,
    pub focused: bool,
    pub focus: Vec<u64>,
    pub nodes: Vec<Node>,
    pub floating_nodes: Vec<Node>,
    pub representation: Option<String>,
    pub fullscreen_mode: Option<NodeFullscreenMode>,
    pub app_id: Option<String>,
    pub pid: Option<u64>,
    pub visible: Option<bool>,
    pub window: Option<u64>,
    pub window_properties: Option<NodeWindowProperties>
}
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NodeType {
    Root,
    Output,
    Workspace,
    Con,
    #[serde(rename = "floating_con")]
    FloatingCon
}
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NodeBorder { Normal, None, Pixel, Csd }
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NodeLayout { None, Splith, Splitv, Stacked, Tabbed, Output }
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum NodeOrientation { Vertical, Horizontal, None }
#[derive(Deserialize_repr, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
#[repr(u8)]
pub enum NodeFullscreenMode { None = 0, Full = 1, Global = 2 }
#[derive(Deserialize, Debug, PartialEq)]
pub struct NodeWindowProperties {
    pub class: String,
    pub instance: String,
    pub title: String
//  pub   transient_for: unknown
}

// Get Bar Config
#[derive(Deserialize, Debug, PartialEq)]
pub struct Bar {
    pub id: String,
    pub mode: BarMode,
    pub position: BarPosition,
    pub status_command: String,
    pub font: String,
    pub workspace_buttons: bool,
    pub binding_mode_indicator: bool,
    pub colors: BarColors,
    pub gaps: BarGaps,
    pub bar_height: u64,
    pub status_padding: u64,
    pub status_edge_padding: u64,
}
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BarMode { Dock, Hide, Invisible }
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BarPosition { Bottom, Top }
#[derive(Deserialize, Debug, PartialEq)]
pub struct BarColors {
    pub background: Color,
    pub statusline: Color,
    pub separator: Color,

    pub focused_background: Color,
    pub focused_statusline: Color,
    pub focused_separator: Color,

    pub focused_workspace_text: Color,
    pub focused_workspace_bg: Color,
    pub focused_workspace_border: Color,

    pub active_workspace_text: Color,
    pub active_workspace_bg: Color,
    pub active_workspace_border: Color,

    pub inactive_workspace_text: Color,
    pub inactive_workspace_bg: Color,
    pub inactive_workspace_border: Color,

    pub urgent_workspace_text: Color,
    pub urgent_workspace_bg: Color,
    pub urgent_workspace_border: Color,

    pub binding_mode_text: Color,
    pub binding_mode_bg: Color,
    pub binding_mode_border: Color,
}
#[derive(Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}
#[derive(Deserialize, Debug, PartialEq)]
pub struct BarGaps {
    pub top: u64,
    pub right: u64,
    pub bottom: u64,
    pub left: u64
}

// Get Version
#[derive(Deserialize, Debug, PartialEq)]
pub struct Version {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub human_readable: String,
    pub loaded_config_file_name: String
}

//Get Config
#[derive(Deserialize, Debug, PartialEq)]
pub struct Config {
    pub config: String
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct BindingState {
    pub name: String
}

// Get Inputs
#[derive(Deserialize, Debug, Default, PartialEq)]
#[serde(default)]
pub struct Input {
    pub identifier: String,
    pub name: String,
    pub vendor: u64,
    pub product: u64,
    #[serde(rename = "type")]
    pub input_type: String,
    #[serde(rename = "xkb_active_layout_name")]
    pub layout: Option<String>,
    #[serde(rename = "xkb_layout_names")]
    pub layouts: Option<Vec<String>>,
    #[serde(rename = "xkb_active_layout_index")]
    pub layout_index: u64,
    pub libinput: InputSettings
}
#[derive(Deserialize, Debug, Default, PartialEq)]
#[serde(default)]
pub struct InputSettings {
    pub send_events: Option<InputSendEvents>,
    #[serde(deserialize_with = "is_enabled")]
    pub tap: Option<bool>,
    #[serde(deserialize_with = "is_enabled")]
    pub tap_button_map: Option<bool>,
    #[serde(deserialize_with = "is_enabled")]
    pub tap_drag: Option<bool>,
    #[serde(deserialize_with = "is_enabled")]
    pub tap_drag_lock: Option<bool>,
    pub accel_speed: Option<f64>,
    pub accel_profile: Option<InputAccelProfile>,
    #[serde(deserialize_with = "is_enabled")]
    pub natural_scroll: Option<bool>,
    #[serde(deserialize_with = "is_enabled")]
    pub left_handed: Option<bool>,
    pub click_method: Option<InputClickMethod>,
    #[serde(deserialize_with = "is_enabled")]
    pub middle_emulation: Option<bool>,
    pub scroll_method: Option<InputScrollMethod>,
    pub scroll_button: Option<u64>,
    #[serde(deserialize_with = "is_enabled")]
    pub dwt: Option<bool>,
    pub calibration_matrix: Option<[f32; 6]>
}
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InputSendEvents {
    Enabled,
    Disabled,
    #[serde(rename = "disabled_on_external_mouse")]
    DisabledOnExternalMouse
}
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InputAccelProfile { None, Flat, Adaptive }
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InputClickMethod {
    None,
    #[serde(rename = "button_areas")]
    ButtonAreas,
    ClickFinger
}
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum InputScrollMethod {
    None,
    #[serde(rename = "two_finger")]
    TwoFinger,
    Edge,
    #[serde(rename = "on_button_down")]
    OnButtonDown,
}

// Get Seats
#[derive(Deserialize, Debug, PartialEq)]
pub struct Seat  {
    pub name: String,
    pub capabilities: u64,
    pub focus: u64,
    pub devices: Vec<Input>
}

// Custom deserializer
impl<'de> serde::de::Deserialize<'de> for Color {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = serde::de::Deserialize::deserialize(deserializer)?;
        //assert!(s[0] == "#", "Invalid colorcode format");
        // Read
        Ok(Color{
            r: u8::from_str_radix(&s[1..2], 16).unwrap(),
            g: u8::from_str_radix(&s[3..4], 16).unwrap(),
            b: u8::from_str_radix(&s[5..6], 16).unwrap(),
            a: u8::from_str_radix(&s[7..8], 16).unwrap()
        })
    }
}

fn is_enabled<'de, D>(deserializer: D) -> Result<std::option::Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = serde::de::Deserialize::deserialize(deserializer)?;
    match s {
        "enabled" => Ok(Some(true)),
        "disabled" => Ok(Some(false)),
        "" => Ok(None),
        _ => panic!("Invalid value in enabled field"),
    }
}
