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
    num: i64,
    name: String,
    visible: bool,
    focused: bool,
    urgent: bool,
    output: String,
    rect: Rectangle
}
#[derive(Deserialize, Debug, PartialEq)]
pub struct Rectangle {
    x: u64,
    y: u64,
    width: u64,
    height: u64
}

// Get Output
#[derive(Deserialize, Debug, PartialEq)]
pub struct Output {
    name: String,
    make: String,
    model: String,
    serial: String,
    active: bool,
    dpms: bool,
    primary: bool,
    scale: f64,
    subpixel_hinting: SubpixelHinting,
    transform: OutputTransform,
    current_workspace: String,
    modes: Vec<OutputMode>,
    current_mode: OutputMode,
    rect: Rectangle
}
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum SubpixelHinting { RGB, BGR, VRGB, VBGR, None, Unknown }
#[derive(Deserialize, Debug, PartialEq)]
enum OutputTransform {
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
    width: u64,
    height: u64,
    refresh: u64
}

// Get Tree
#[derive(Deserialize, Debug, PartialEq)]
pub struct Node {
    id: u64,
    name: String,
    #[serde(rename = "type")]
    node_type: NodeType,
    border: NodeBorder,
    current_border_width: u8,
    layout: NodeLayout,
    orientation: NodeOrientation,
    //percent: Option<f64>,
    rect: Rectangle,
    window_rect: Rectangle,
    deco_rect: Rectangle,
    geometry: Rectangle,
    urgent: bool,
    sticky: bool,
    focused: bool,
    focus: Vec<u64>,
    nodes: Vec<Node>,
    floating_nodes: Vec<Node>,
    representation: Option<String>,
    fullscreen_mode: Option<NodeFullscreenMode>,
    app_id: Option<String>,
    pid: Option<u64>,
    visible: Option<bool>,
    window: Option<u64>,
    window_properties: Option<NodeWindowProperties>
}
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum NodeType {
    Root,
    Output,
    Workspace,
    Con,
    #[serde(rename = "floating_con")]
    FloatingCon
}
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum NodeBorder { Normal, None, Pixel, Csd }
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum NodeLayout { None, Splith, Splitv, Stacked, Tabbed, Output }
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum NodeOrientation { Vertical, Horizontal, None }
#[derive(Deserialize_repr, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
#[repr(u8)]
enum NodeFullscreenMode { None = 0, Full = 1, Global = 2 }
#[derive(Deserialize, Debug, PartialEq)]
pub struct NodeWindowProperties {
    class: String,
    instance: String,
    title: String
//    transient_for: unknown
}

// Get Bar Config
#[derive(Deserialize, Debug, PartialEq)]
pub struct Bar {
    id: String,
    mode: BarMode,
    position: BarPosition,
    status_command: String,
    font: String,
    workspace_buttons: bool,
    binding_mode_indicator: bool,
    colors: BarColors,
    gaps: BarGaps,
    bar_height: u64,
    status_padding: u64,
    status_edge_padding: u64,
}
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum BarMode { Dock, Hide, Invisible }
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
enum BarPosition { Bottom, Top }
#[derive(Deserialize, Debug, PartialEq)]
pub struct BarColors {
    background: Color,
    statusline: Color,
    separator: Color,

    focused_background: Color,
    focused_statusline: Color,
    focused_separator: Color,

    focused_workspace_text: Color,
    focused_workspace_bg: Color,
    focused_workspace_border: Color,

    active_workspace_text: Color,
    active_workspace_bg: Color,
    active_workspace_border: Color,

    inactive_workspace_text: Color,
    inactive_workspace_bg: Color,
    inactive_workspace_border: Color,

    urgent_workspace_text: Color,
    urgent_workspace_bg: Color,
    urgent_workspace_border: Color,

    binding_mode_text: Color,
    binding_mode_bg: Color,
    binding_mode_border: Color,
}
#[derive(Debug, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}
#[derive(Deserialize, Debug, PartialEq)]
pub struct BarGaps {
    top: u64,
    right: u64,
    bottom: u64,
    left: u64
}

// Get Version
#[derive(Deserialize, Debug, PartialEq)]
pub struct Version {
    major: u64,
    minor: u64,
    patch: u64,
    human_readable: String,
    loaded_config_file_name: String
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
    identifier: String,
    name: String,
    vendor: u64,
    product: u64,
    #[serde(rename = "type")]
    input_type: String,
    #[serde(rename = "xkb_active_layout_name")]
    layout: Option<String>,
    #[serde(rename = "xkb_layout_names")]
    layouts: Option<Vec<String>>,
    #[serde(rename = "xkb_active_layout_index")]
    layout_index: u64,
    libinput: InputSettings
}
#[derive(Deserialize, Debug, Default, PartialEq)]
#[serde(default)]
pub struct InputSettings {
    send_events: Option<InputSendEvents>,
    #[serde(deserialize_with = "is_enabled")]
    tap: Option<bool>,
    #[serde(deserialize_with = "is_enabled")]
    tap_button_map: Option<bool>,
    #[serde(deserialize_with = "is_enabled")]
    tap_drag: Option<bool>,
    #[serde(deserialize_with = "is_enabled")]
    tap_drag_lock: Option<bool>,
    accel_speed: Option<f64>,
    accel_profile: Option<InputAccelProfile>,
    #[serde(deserialize_with = "is_enabled")]
    natural_scroll: Option<bool>,
    #[serde(deserialize_with = "is_enabled")]
    left_handed: Option<bool>,
    click_method: Option<InputClickMethod>,
    #[serde(deserialize_with = "is_enabled")]
    middle_emulation: Option<bool>,
    scroll_method: Option<InputScrollMethod>,
    scroll_button: Option<u64>,
    #[serde(deserialize_with = "is_enabled")]
    dwt: Option<bool>,
    calibration_matrix: Option<[f32; 6]>
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
    name: String,
    capabilities: u64,
    focus: u64,
    devices: Vec<Input>
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
