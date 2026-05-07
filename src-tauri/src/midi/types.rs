use serde::{Deserialize, Serialize};

// ── Enum mirrors (values must match preset_types.h) ─────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[repr(i32)]
pub enum MsgType {
    #[default]
    None = 0,
    Pc = 1,
    Cc = 2,
    NoteOn = 3,
    NoteOff = 4,
    ClockBpm = 5,
    ClockTap = 6,
    Sysex = 7,
    Mmc = 8,
    Realtime = 9,
    Relay = 10,
    BankJump = 11,
    PageJump = 12,
    PresetToggle = 13,
    Keystroke = 14,
}

impl MsgType {
    fn from_i32(v: i32) -> Self {
        match v {
            1 => Self::Pc,
            2 => Self::Cc,
            3 => Self::NoteOn,
            4 => Self::NoteOff,
            5 => Self::ClockBpm,
            6 => Self::ClockTap,
            7 => Self::Sysex,
            8 => Self::Mmc,
            9 => Self::Realtime,
            10 => Self::Relay,
            11 => Self::BankJump,
            12 => Self::PageJump,
            13 => Self::PresetToggle,
            14 => Self::Keystroke,
            _ => Self::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[repr(i32)]
pub enum Action {
    #[default]
    Press = 0,
    Release = 1,
    DoubleTap = 2,
    LongPress = 3,
    LongPressRel = 4,
    DoubleTapRel = 5,
}

impl Action {
    fn from_i32(v: i32) -> Self {
        match v {
            1 => Self::Release,
            2 => Self::DoubleTap,
            3 => Self::LongPress,
            4 => Self::LongPressRel,
            5 => Self::DoubleTapRel,
            _ => Self::Press,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[repr(i32)]
pub enum TogglePos {
    #[default]
    Pos1 = 0,
    Pos2 = 1,
    Shift = 2,
}

impl TogglePos {
    pub fn from_i32(v: i32) -> Self {
        match v {
            1 => Self::Pos2,
            2 => Self::Shift,
            _ => Self::Pos1,
        }
    }
}

// ── Serialisable Rust types ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MidiMsg {
    pub msg_type: MsgType,
    pub action: Action,
    pub toggle_pos: TogglePos,
    pub port: u8,
    pub channel: u8,
    pub number: u8,
    pub value: u8,
    pub sysex: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    pub name: String,
    pub color_pos1: [u8; 3],
    pub color_pos2: [u8; 3],
    pub toggle_group: u8,
    pub toggle_reset_group: u8,
    pub msgs: Vec<MidiMsg>,
}

impl Default for Preset {
    fn default() -> Self {
        Self {
            name: String::new(),
            color_pos1: [0, 0, 0],
            color_pos2: [0, 0, 0],
            toggle_group: 0,
            toggle_reset_group: 0,
            msgs: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Bank {
    pub name: String,
    /// [page][switch] — 4 pages, 4 switches each
    pub presets: [[Preset; 4]; 4],
    pub expr_presets: [Preset; 2],
    pub enter_msgs: Vec<MidiMsg>,
    pub exit_msgs: Vec<MidiMsg>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeviceSettings {
    pub brightness: u8,
    pub startup_bank: u8,
    pub startup_profile: u8,
    pub midi_thru_usb: u8,
    pub midi_thru_trs: u8,
    pub relay_contact: [u8; 2],
    pub recall_toggle: bool,
    pub device_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub firmware_major: u8,
    pub firmware_minor: u8,
    pub firmware_patch: u8,
    pub device_name: String,
}

// ── Binary deserialisation ───────────────────────────────────────────────────
// C struct layout for ESP32 (ARM32, default ABI, int-sized enums):
//   midi_msg_t : 52 bytes  (3 × i32 enums + 5 × u8 + 3 pad + u8[32])
//   preset_t   : 1692 bytes
//   bank_t     : 33812 bytes
//   device_settings_t : 29 bytes

pub const MIDI_MSG_SIZE: usize = 52;
pub const PRESET_SIZE: usize = 1692;
pub const BANK_SIZE: usize = 33812;
pub const DEVICE_SETTINGS_SIZE: usize = 29;

fn read_i32_le(buf: &[u8], offset: usize) -> i32 {
    i32::from_le_bytes(buf[offset..offset + 4].try_into().unwrap())
}

fn write_i32_le(buf: &mut [u8], offset: usize, v: i32) {
    buf[offset..offset + 4].copy_from_slice(&v.to_le_bytes());
}

pub fn deserialise_midi_msg(buf: &[u8]) -> MidiMsg {
    let msg_type = MsgType::from_i32(read_i32_le(buf, 0));
    let action = Action::from_i32(read_i32_le(buf, 4));
    let toggle_pos = TogglePos::from_i32(read_i32_le(buf, 8));
    let port = buf[12];
    let channel = buf[13];
    let number = buf[14];
    let value = buf[15];
    let sysex_len = buf[16] as usize;
    // buf[17..20] = padding
    let sysex = buf[20..20 + sysex_len.min(32)].to_vec();
    MidiMsg { msg_type, action, toggle_pos, port, channel, number, value, sysex }
}

pub fn serialise_midi_msg(msg: &MidiMsg, buf: &mut [u8]) {
    buf[..MIDI_MSG_SIZE].fill(0);
    write_i32_le(buf, 0, msg.msg_type as i32);
    write_i32_le(buf, 4, msg.action as i32);
    write_i32_le(buf, 8, msg.toggle_pos as i32);
    buf[12] = msg.port;
    buf[13] = msg.channel;
    buf[14] = msg.number;
    buf[15] = msg.value;
    let sysex_len = msg.sysex.len().min(32);
    buf[16] = sysex_len as u8;
    buf[20..20 + sysex_len].copy_from_slice(&msg.sysex[..sysex_len]);
}

pub fn deserialise_preset(buf: &[u8]) -> Preset {
    let name = cstr_to_string(&buf[0..17]);
    let color_pos1 = [buf[17], buf[18], buf[19]];
    let color_pos2 = [buf[20], buf[21], buf[22]];
    let toggle_group = buf[23];
    let toggle_reset_group = buf[24];
    let num_msgs = buf[25] as usize;
    // buf[26..28] = padding
    let msgs = (0..num_msgs.min(32))
        .map(|i| deserialise_midi_msg(&buf[28 + i * MIDI_MSG_SIZE..]))
        .collect();
    Preset { name, color_pos1, color_pos2, toggle_group, toggle_reset_group, msgs }
}

pub fn serialise_preset(preset: &Preset, buf: &mut [u8]) {
    buf[..PRESET_SIZE].fill(0);
    string_to_cstr(&preset.name, &mut buf[0..17]);
    buf[17..20].copy_from_slice(&preset.color_pos1);
    buf[20..23].copy_from_slice(&preset.color_pos2);
    buf[23] = preset.toggle_group;
    buf[24] = preset.toggle_reset_group;
    let num_msgs = preset.msgs.len().min(32);
    buf[25] = num_msgs as u8;
    for (i, msg) in preset.msgs.iter().take(32).enumerate() {
        serialise_midi_msg(msg, &mut buf[28 + i * MIDI_MSG_SIZE..]);
    }
}

pub fn deserialise_bank(buf: &[u8]) -> Bank {
    let name = cstr_to_string(&buf[0..17]);
    // buf[17..20] = padding
    let presets = std::array::from_fn(|page| {
        std::array::from_fn(|sw| {
            let off = 20 + (page * 4 + sw) * PRESET_SIZE;
            deserialise_preset(&buf[off..])
        })
    });
    let expr_presets = std::array::from_fn(|i| {
        let off = 20 + 16 * PRESET_SIZE + i * PRESET_SIZE;
        deserialise_preset(&buf[off..])
    });
    // After presets+expr_presets: offset 20 + 18*PRESET_SIZE = 20 + 30456 = 30476
    let num_enter = buf[30476] as usize;
    // buf[30477..30480] = padding
    let enter_msgs = (0..num_enter.min(32))
        .map(|i| deserialise_midi_msg(&buf[30480 + i * MIDI_MSG_SIZE..]))
        .collect();
    let num_exit = buf[32144] as usize;
    // buf[32145..32148] = padding
    let exit_msgs = (0..num_exit.min(32))
        .map(|i| deserialise_midi_msg(&buf[32148 + i * MIDI_MSG_SIZE..]))
        .collect();
    Bank { name, presets, expr_presets, enter_msgs, exit_msgs }
}

pub fn serialise_bank(bank: &Bank) -> Vec<u8> {
    let mut buf = vec![0u8; BANK_SIZE];
    string_to_cstr(&bank.name, &mut buf[0..17]);
    for page in 0..4 {
        for sw in 0..4 {
            let off = 20 + (page * 4 + sw) * PRESET_SIZE;
            serialise_preset(&bank.presets[page][sw], &mut buf[off..]);
        }
    }
    for i in 0..2 {
        let off = 20 + 16 * PRESET_SIZE + i * PRESET_SIZE;
        serialise_preset(&bank.expr_presets[i], &mut buf[off..]);
    }
    let num_enter = bank.enter_msgs.len().min(32);
    buf[30476] = num_enter as u8;
    for (i, msg) in bank.enter_msgs.iter().take(32).enumerate() {
        serialise_midi_msg(msg, &mut buf[30480 + i * MIDI_MSG_SIZE..]);
    }
    let num_exit = bank.exit_msgs.len().min(32);
    buf[32144] = num_exit as u8;
    for (i, msg) in bank.exit_msgs.iter().take(32).enumerate() {
        serialise_midi_msg(msg, &mut buf[32148 + i * MIDI_MSG_SIZE..]);
    }
    buf
}

pub fn deserialise_settings(buf: &[u8]) -> DeviceSettings {
    DeviceSettings {
        brightness: buf[0],
        startup_bank: buf[1],
        startup_profile: buf[2],
        midi_thru_usb: buf[3],
        midi_thru_trs: buf[4],
        relay_contact: [buf[5], buf[6]],
        recall_toggle: buf[7] != 0,
        device_name: cstr_to_string(&buf[8..29]),
    }
}

pub fn serialise_settings(s: &DeviceSettings) -> Vec<u8> {
    let mut buf = vec![0u8; DEVICE_SETTINGS_SIZE];
    buf[0] = s.brightness;
    buf[1] = s.startup_bank;
    buf[2] = s.startup_profile;
    buf[3] = s.midi_thru_usb;
    buf[4] = s.midi_thru_trs;
    buf[5] = s.relay_contact[0];
    buf[6] = s.relay_contact[1];
    buf[7] = s.recall_toggle as u8;
    string_to_cstr(&s.device_name, &mut buf[8..29]);
    buf
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn cstr_to_string(buf: &[u8]) -> String {
    let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
    String::from_utf8_lossy(&buf[..end]).into_owned()
}

fn string_to_cstr(s: &str, buf: &mut [u8]) {
    buf.fill(0);
    let bytes = s.as_bytes();
    let len = bytes.len().min(buf.len().saturating_sub(1));
    buf[..len].copy_from_slice(&bytes[..len]);
}
