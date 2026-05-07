use tokio::sync::broadcast;

use crate::midi::connection::MidiConnection;
use crate::midi::sysex::SysexFrame;
use crate::midi::types::{Bank, DeviceSettings, Preset};

pub const MOCK_DEVICE_NAME: &str = "Daisy Mock Device";
pub const MOCK_PROFILE_COUNT: usize = 2;

pub struct AppState {
    pub connection: Option<MidiConnection>,
    pub seq: u8,
    pub mock: bool,
    pub mock_banks: Vec<Vec<Bank>>,
    pub mock_settings: DeviceSettings,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            connection: None,
            seq: 0,
            mock: false,
            mock_banks: (0..MOCK_PROFILE_COUNT).map(|p| make_mock_banks(p as u8)).collect(),
            mock_settings: DeviceSettings {
                brightness: 80,
                startup_bank: 0,
                startup_profile: 0,
                midi_thru_usb: 0,
                midi_thru_trs: 0,
                relay_contact: [0, 0],
                recall_toggle: false,
                device_name: "Daisy Mock".to_string(),
            },
        }
    }

    pub fn protocol_parts(
        &mut self,
    ) -> Result<(&mut midir::MidiOutputConnection, broadcast::Sender<SysexFrame>, &mut u8), String> {
        let conn = self
            .connection
            .as_mut()
            .ok_or_else(|| "Not connected to a MIDI device".to_string())?;
        let frame_tx = conn.frame_tx.clone();
        Ok((&mut conn.output, frame_tx, &mut self.seq))
    }
}

fn make_preset(name: &str, r: u8, g: u8, b: u8) -> Preset {
    Preset {
        name: name.to_string(),
        color_pos1: [r, g, b],
        color_pos2: [r / 3, g / 3, b / 3],
        ..Default::default()
    }
}

fn make_mock_banks(profile: u8) -> Vec<Bank> {
    let bank_templates: &[(&str, [[(&str, [u8; 3]); 4]; 4])] = &[
        (
            "Overdrives",
            [
                [("Tube Scream", [255, 80, 0]), ("Blues OD", [255, 140, 0]), ("Rat", [200, 40, 0]), ("Klone", [180, 160, 0])],
                [("Big Muff", [160, 0, 200]), ("Fuzz Face", [120, 0, 180]), ("Tonebender", [100, 0, 160]), ("Velvet", [80, 0, 140])],
                [("Boost", [0, 180, 60]), ("Clean Boost", [0, 200, 80]), ("EP Boost", [0, 160, 40]), ("LPB-1", [0, 140, 20])],
                [("Compressor", [0, 100, 200]), ("Sustain", [0, 80, 180]), ("Squeeze", [0, 60, 160]), ("Optical", [0, 40, 140])],
            ],
        ),
        (
            "Modulation",
            [
                [("Chorus", [0, 160, 200]), ("Ensemble", [0, 140, 180]), ("CE-1", [0, 120, 160]), ("Flanger", [0, 100, 140])],
                [("Phaser", [200, 0, 160]), ("Phase 90", [180, 0, 140]), ("Bi-Phase", [160, 0, 120]), ("Small Stone", [140, 0, 100])],
                [("Tremolo", [200, 160, 0]), ("Vibrato", [180, 140, 0]), ("UniVibe", [160, 120, 0]), ("Rotary", [140, 100, 0])],
                [("Ring Mod", [0, 200, 120]), ("Pitch Shift", [0, 180, 100]), ("Whammy", [0, 160, 80]), ("Harmonist", [0, 140, 60])],
            ],
        ),
        (
            "Delays",
            [
                [("Digital Delay", [0, 120, 220]), ("Analog Delay", [0, 100, 200]), ("Tape Echo", [0, 80, 180]), ("Reverse", [0, 60, 160])],
                [("Slapback", [220, 120, 0]), ("Ping Pong", [200, 100, 0]), ("Multi-Tap", [180, 80, 0]), ("Looper", [160, 60, 0])],
                [("Shimmer", [160, 0, 220]), ("Modulated", [140, 0, 200]), ("Ducked", [120, 0, 180]), ("Dotted 8th", [100, 0, 160])],
                [("Quarter", [0, 220, 160]), ("8th Note", [0, 200, 140]), ("Triplet", [0, 180, 120]), ("Half Note", [0, 160, 100])],
            ],
        ),
        (
            "Reverbs",
            [
                [("Hall", [80, 80, 220]), ("Room", [60, 60, 200]), ("Plate", [40, 40, 180]), ("Spring", [20, 20, 160])],
                [("Shimmer Rev", [200, 80, 80]), ("Bloom", [180, 60, 60]), ("Cathedral", [160, 40, 40]), ("Cavern", [140, 20, 20])],
                [("Ambient", [80, 200, 80]), ("Swell", [60, 180, 60]), ("Freeze", [40, 160, 40]), ("Sustain", [20, 140, 20])],
                [("Church", [200, 200, 80]), ("Stadium", [180, 180, 60]), ("Garage", [160, 160, 40]), ("Bathroom", [140, 140, 20])],
            ],
        ),
        (
            "Utilities",
            [
                [("Tuner", [200, 200, 200]), ("Mute", [100, 100, 100]), ("Kill Dry", [80, 80, 80]), ("Blend", [60, 60, 60])],
                [("Tap Tempo", [0, 200, 200]), ("Clock", [0, 180, 180]), ("MIDI Sync", [0, 160, 160]), ("BPM Set", [0, 140, 140])],
                [("Bank Up", [0, 200, 0]), ("Bank Dn", [200, 0, 0]), ("Page Up", [0, 180, 0]), ("Page Dn", [180, 0, 0])],
                [("Loop Rec", [200, 0, 200]), ("Loop Play", [180, 0, 180]), ("Loop Dub", [160, 0, 160]), ("Loop Stop", [140, 0, 140])],
            ],
        ),
        ("Bank 6", [[("Preset 1", [100, 100, 200]), ("Preset 2", [80, 80, 180]), ("Preset 3", [60, 60, 160]), ("Preset 4", [40, 40, 140])]; 4]),
        ("Bank 7", [[("Preset 1", [200, 100, 100]), ("Preset 2", [180, 80, 80]), ("Preset 3", [160, 60, 60]), ("Preset 4", [140, 40, 40])]; 4]),
        ("Bank 8", [[("Preset 1", [100, 200, 100]), ("Preset 2", [80, 180, 80]), ("Preset 3", [60, 160, 60]), ("Preset 4", [40, 140, 40])]; 4]),
    ];

    bank_templates
        .iter()
        .enumerate()
        .map(|(i, (name, pages))| {
            let bank_name = if profile == 0 {
                name.to_string()
            } else {
                format!("B {}", i + 1)
            };
            Bank {
                name: bank_name,
                presets: std::array::from_fn(|page| {
                    std::array::from_fn(|sw| {
                        let (preset_name, color) = pages[page][sw];
                        make_preset(preset_name, color[0], color[1], color[2])
                    })
                }),
                ..Default::default()
            }
        })
        .collect()
}
