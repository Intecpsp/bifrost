#![allow(clippy::pub_underscore_fields, clippy::struct_excessive_bools)]

use serde::{Deserialize, Serialize, ser::SerializeSeq};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::net::IpAddr;

use crate::serde_util::{bool_as_int, option_ipaddr_or_empty};
use crate::types::{OptFlags, SegCap};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WledInfo {
    /// Version string
    pub ver: String,

    /// Build ID (YYMMDDB, B = daily build index).
    pub vid: u64,

    /// Codename
    pub cn: String,

    /// Release string
    pub release: String,

    /// Contains info about the LED setup.
    pub leds: Leds,

    // i2c (debug only)

    // spi (debug only)
    /// If true, an UI with only a single button for toggling sync should toggle
    /// receive+send, otherwise send only
    #[serde(rename = "str")]
    pub sync_toggle_receive: bool,

    /// Friendly name of the light. Intended for display in lists and titles.
    pub name: String,

    /// The UDP port for realtime packets and WLED broadcast.
    pub udpport: u16,

    /// Simplified UI enabled?
    pub simplifiedui: bool,

    /// If true, the software is currently receiving realtime data via UDP or E1.31.
    pub live: bool,

    /// Main segment id for live streaming, or -1 if not using main segment only
    pub liveseg: i8,

    /// Realtime mode description
    ///
    /// Empty string means inactive or generic mode.
    pub lm: String,

    /// Address that is live streaming
    ///
    /// `None` when not streaming.
    #[serde(with = "option_ipaddr_or_empty")]
    pub lip: Option<IpAddr>,

    /// Number of currently connected websocket clients. -1 indicates that WS is unsupported in this build.
    ///
    /// Value range: `-1` to `8`
    pub ws: i8,

    /// Number of effects included.
    pub fxcount: u16,

    /// Number of palettes configured.
    pub palcount: u16,

    /// Number of custom palettes.
    pub cpalcount: u16,

    pub maps: Vec<WledInfoMap>,

    /// Wifi information
    pub wifi: WledInfoWifi,

    /// Filesystem information
    pub fs: WledInfoFs,

    /// Number of other WLED devices discovered on the network. -1 if Node discovery disabled. (since 0.12.0)
    ///
    /// Value range: `-1` to `255`
    pub ndc: i16,

    /// Name of the platform.
    pub arch: String,

    /// Version of the underlying (Arduino core) SDK.
    pub core: String,

    /// Board cpu frequency (MHz)
    pub clock: u16,

    /// Board flash size (MB)
    pub flash: u8,

    #[deprecated]
    #[serde(default)]
    pub lwip: u8,

    pub freeheap: u64,

    pub uptime: u64,

    pub time: String,

    /// Used for debugging purposes only.
    pub opt: OptFlags,

    /// The producer/vendor of the light. Always WLED for standard installations.
    pub brand: String,

    /// The product name. Always FOSS for standard installations.
    pub product: String,

    /// The hexadecimal hardware MAC address of the light, lowercase and without colons.
    pub mac: String,

    /// The IP address of this instance. Empty string if not connected. (since 0.13.0)
    #[serde(with = "option_ipaddr_or_empty")]
    pub ip: Option<IpAddr>,

    /// Usermod values
    #[serde(default)]
    pub u: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WledInfoMap {
    pub id: u8,
    #[serde(default)]
    pub n: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WledInfoFs {
    pub u: u32,
    pub t: u32,
    pub pmt: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WledInfoWifi {
    pub bssid: String,
    pub rssi: i32,
    pub signal: u32,
    pub channel: u16,
    pub ap: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Leds {
    /// Total led count
    pub count: u32,

    /// Current current (mA)
    pub pwr: u32,

    /// Frames per second
    pub fps: u32,

    /// Maximum current (mA) (0 if current limit not in use)
    pub maxpwr: u32,

    /// Maximum supported segments
    pub maxseg: u32,

    /// Boot preset
    pub bootps: u32,

    /// Light capabilities per segment
    pub seglc: Vec<SegCap>,

    /// Total supported features by any segment
    ///
    /// NOTE: Official documentation states that this is the AND of all segment
    /// LC values, but this is in fact the OR of them.
    ///
    /// In other words, this is the set of capabilities supported by *at least*
    /// one segment.
    pub lc: SegCap,

    /// True if any segment has a
    #[deprecated = "use .lc instead"]
    pub rgbw: bool,

    /// True if any segment has a white channel
    #[deprecated = "use .lc instead"]
    #[serde(with = "bool_as_int")]
    pub wv: bool,

    /// True if any segment supports cct
    #[deprecated = "use .lc instead"]
    #[serde(with = "bool_as_int")]
    pub cct: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WledState {
    /// Brightness of the light. If on is false, contains last brightness when
    /// light was on (aka brightness when on is set to true).
    ///
    /// Setting bri to 0 is supported but it is recommended to use the range
    /// 1-255 and use on: false to turn off.
    ///
    /// The state response will never have the value 0 for bri.
    pub bri: u8,

    /// On/Off state of the light. You can also use "t" instead of true or false to toggle.
    pub on: bool,

    /// ID of currently set playlist. (read-olny)
    pub pl: i16,

    /// Segments are individual parts of the LED strip. Since 0.9.0 this enables
    /// running different effects on different parts of the strip.
    pub seg: Vec<StateSeg>,

    /// Main Segment
    pub mainseg: u8,

    /// Duration of the crossfade between different colors/brightness levels.
    ///
    /// One unit is 100ms, so a value of 4 results in atransition of 400ms.
    pub transition: i32,

    /// Load specified ledmap (0 for ledmap.json, 1-9 for ledmap1.json to
    /// ledmap9.json). See mapping. (available since 0.14.0)
    pub ledmap: u8,

    pub udpn: StateUdpn,

    pub ps: i16,

    /// Live data override. 0 is off, 1 is override until live data ends, 2 is
    /// override until ESP reboot (available since 0.10.0)
    pub lor: u8,

    /// Nightlight settings
    pub nl: NightLight,

    #[serde(rename = "AudioReactive")]
    pub audio_reactive: AudioReactive,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum NightLightMode {
    Instant = 0,
    Fade = 1,
    ColorFade = 2,
    Sunrise = 3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NightLight {
    /// Nightlight currently active
    pub on: bool,

    /// Duration of nightlight in minutes
    ///
    /// Value range: `1` to `255`
    pub dur: u8,

    /// Nightlight mode (0: instant, 1: fade, 2: color fade, 3: sunrise)
    /// (available since 0.10.2)
    ///
    /// Value range: `0` to `3`
    pub mode: NightLightMode,

    /// Target brightness of nightlight feature
    ///
    /// Value range: `0` to `255`
    pub tbri: u8,

    /// Remaining nightlight duration in seconds, -1 if not active. Only in
    /// state response, can not be set.
    ///
    /// Value range: `-1` to `15300`
    pub rem: i16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioReactive {
    pub on: bool,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StateUpdate {
    pub bri: Option<u8>,
    pub on: Option<bool>,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Expand1dFx {
    Pixels = 0,
    Bar = 1,
    Arc = 2,
    Corner = 3,
    Pinwheel = 4,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SoundSim {
    BeatSin = 0,
    WeWillRockYou = 1,
    Sim10_13 = 2,
    Sim14_3 = 3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateSeg {
    /// Array that has up to 3 color arrays as elements, the primary, secondary
    /// (background) and tertiary colors of the segment.
    ///
    /// Each color is an array of 3 or 4 bytes, which represents a RGB(W) color,
    /// i.e. `[[255,170,0],[0,0,0],[64,64,64]]`.
    ///
    /// It can also be represented as an array of strings of hex values,
    /// i.e:
    ///
    /// `["FFAA00", "000000", "404040"]` (for orange, black and grey.)
    pub col: Colors,

    /// Sets the individual segment brightness (available since 0.10.0)
    ///
    /// Value range: `0` to `255`
    pub bri: u8,

    /// Effect custom slider 1. Custom sliders are hidden or displayed and
    /// labeled based on effect metadata.
    ///
    /// Value range: `0` to `255`
    pub c1: u8,

    /// Effect custom slider 2.
    ///
    /// Value range: `0` to `255`
    pub c2: u8,

    /// Effect custom slider 3.
    ///
    /// Value range: `0` to `31`
    pub c3: u8,

    /// White spectrum color temperature (available since 0.13.0)
    ///
    /// Value range: `0` to `255`, `1900` to `10091`
    pub cct: u16,

    /// freezes/unfreezes the current effect
    pub frz: bool,

    /// ID of the effect or `~` to increment, `~-` to decrement, or `r` for random.
    ///
    /// Value range: `0` to `info.fxcount - 1`
    pub fx: u8,

    /// Grouping (how many consecutive LEDs of the same segment will be
    /// grouped to the same color)
    ///
    /// Value range: `0` to `255`
    pub grp: u8,

    /// Zero-indexed ID of the segment. May be omitted, in that case the ID will
    /// be inferred from the order of the segment objects in the seg array.
    ///
    /// Value range: `0` to `info.maxseg - 1`
    pub id: u8,

    /// Effect intensity. `~` to increment, `~-` to decrement.
    /// `~10` to increment by 10.
    /// `~-10` to decrement by 10.
    ///
    /// Value range: `0` to `255`
    pub ix: u8,

    /// Length of the segment (stop - start). stop has preference, so if it is
    /// included, len is ignored.
    ///
    /// Value range: `0` to `info.leds.count`
    pub len: u8,

    /// Setting of segment field 'Expand 1D FX'
    pub m12: Expand1dFx,

    /// Mirrors the segment (in horizontal dimension for 2D set-up) (available
    /// since 0.10.2)
    pub mi: bool,

    /// Effect option 1. Custom options are hidden or displayed and labeled
    /// based on effect metadata.
    pub o1: bool,

    /// Effect option 2.
    pub o2: bool,

    /// Effect option 3.
    pub o3: bool,

    /// Offset (how many LEDs to rotate the virtual start of the segments, available since 0.13.0)
    ///
    /// Value range: `-len + 1` to `len`
    pub of: i16,

    /// Turns on and off the individual segment. (available since 0.10.0)
    pub on: bool,

    /// ID of the color palette or `~` to increment, `~-` to decrement, or `r` for random.
    ///
    /// Value range: `0` to `info.palcount - 1`
    pub pal: u8,

    /// Flips the segment (in horizontal dimension for 2D set-up), causing
    /// animations to change direction.
    pub rev: bool,

    /// `true` if the segment is selected.
    ///
    /// Selected segments will have their state (color/FX) updated by APIs that
    /// don't support segments (e.g. UDP sync, HTTP API).
    ///
    /// If no segment is selected, the first segment (id:0) will behave as if
    /// selected.
    ///
    /// WLED will report the state of the first (lowest id) segment that is
    /// selected to APIs (HTTP, MQTT, Blynk...), or mainseg in case no segment
    /// is selected and for the UDP API.
    ///
    /// Live data is always applied to all LEDs regardless of segment
    /// configuration.
    pub sel: bool,

    /// Assigns group or set ID to segment (not to be confused with
    /// grouping). Visual aid only (helps in UI). (available since 0.14.0)
    ///
    /// Value range: `0` to `3`
    pub set: u8,

    /// Setting of the sound simulation type for audio enhanced effects.
    pub si: SoundSim,

    /// Spacing (how many LEDs are turned off and skipped between each group)
    ///
    /// Value range: 0 to 255
    pub spc: u8,

    /// LED the segment starts at. For 2D set-up it determines column where
    /// segment starts, from top-left corner of the matrix.
    ///
    /// Value range: 0 to `info.leds.count` - 1
    pub start: u8,

    /// LED the segment stops at, not included in range. If stop is set to a
    /// lower or equal value than start (setting to 0 is recommended), the
    /// segment is invalidated and deleted. For 2D set-up it determines column
    /// where segment stops, from top-left corner of the matrix.
    ///
    /// Value range: 0 to info.leds.count
    pub stop: u8,

    /// Relative effect speed. `~` to increment, `~-` to decrement.
    /// `~10` to increment by 10.
    /// `~-10` to decrement by 10.
    ///
    /// Value range: `0` to `255`
    pub sx: u8,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct StateSegUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub col: Option<Colors>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bri: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub on: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fx: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sx: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ix: Option<u8>,

    /// Duration of the crossfade between different colors/brightness
    /// levels. One unit is 100ms, so a value of 4 results in a transition of
    /// 400ms.
    ///
    /// Value range: 0 to 65535
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition: Option<u16>,

    /// Similar to transition, but applies to just the current API call.
    ///
    /// Value range: 0 to 65535
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tt: Option<u16>,

    /// Individual LED control. (available since 0.10.2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i: Option<Vec<Color>>,

    /// Save current light config (state) to specified preset slot.
    ///
    /// Value range: `1` to `250` (16 prior to 0.11)
    pub psave: Option<u16>,
}

/// WLED UDP state broadcast settings
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct StateUdpn {
    /// Send WLED broadcast (UDP sync) packet on state change
    pub send: bool,
    /// Receive broadcast packets
    pub recv: bool,
    /// Bitfield for broadcast send groups 1-8
    pub sgrp: u8,
    /// Bitfield for broadcast receive groups 1-8
    pub rgrp: u8,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Color {
    #[serde(untagged)]
    None([u8; 0]),
    #[serde(untagged)]
    Rgb([u8; 3]),
    #[serde(untagged)]
    Rgbw([u8; 4]),
}

impl Color {
    pub const NONE: Self = Self::None([]);
}

#[derive(Debug, Clone)]
pub struct Colors {
    pub primary: Color,
    pub background: Color,
    pub custom: Color,
}

impl Serialize for Colors {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_seq(Some(3))?;
        map.serialize_element(&self.primary)?;
        map.serialize_element(&self.background)?;
        map.serialize_element(&self.custom)?;
        map.end()
    }
}

impl<'de> Deserialize<'de> for Colors {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let [primary, background, custom] = <[Color; 3]>::deserialize(deserializer)?;
        Ok(Self {
            primary,
            background,
            custom,
        })
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct WledFrame {
    pub info: WledInfo,
    pub state: WledState,
}
