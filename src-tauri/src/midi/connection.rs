use midir::{MidiInput, MidiOutput, MidiOutputConnection};
use tokio::sync::broadcast;

use super::sysex::{parse_frame, SysexFrame};

pub struct MidiConnection {
    pub output: MidiOutputConnection,
    pub frame_tx: broadcast::Sender<SysexFrame>,
    _input: midir::MidiInputConnection<()>,
}

pub fn list_ports() -> Result<Vec<String>, String> {
    let midi_out = MidiOutput::new("daisy-editor-scan").map_err(|e| e.to_string())?;
    let ports = midi_out.ports();
    Ok(ports
        .iter()
        .filter_map(|p| midi_out.port_name(p).ok())
        .collect())
}

pub fn connect(port_name: &str) -> Result<MidiConnection, String> {
    let midi_out = MidiOutput::new("daisy-editor-out").map_err(|e| e.to_string())?;
    let out_port = midi_out
        .ports()
        .into_iter()
        .find(|p| midi_out.port_name(p).ok().as_deref() == Some(port_name))
        .ok_or_else(|| format!("MIDI output port not found: {port_name}"))?;
    let output = midi_out
        .connect(&out_port, "daisy-editor-out")
        .map_err(|e| e.to_string())?;

    let midi_in = MidiInput::new("daisy-editor-in").map_err(|e| e.to_string())?;
    let in_port = midi_in
        .ports()
        .into_iter()
        .find(|p| midi_in.port_name(p).ok().as_deref() == Some(port_name))
        .ok_or_else(|| format!("MIDI input port not found: {port_name}"))?;

    let (frame_tx, _) = broadcast::channel::<SysexFrame>(64);
    let tx = frame_tx.clone();

    let _input = midi_in
        .connect(
            &in_port,
            "daisy-editor-in",
            {
                let mut sysex_buf: Vec<u8> = Vec::new();
                let mut in_sysex = false;
                move |_stamp, msg, _| {
                    for &byte in msg {
                        if byte == 0xF0 {
                            sysex_buf.clear();
                            sysex_buf.push(byte);
                            in_sysex = true;
                        } else if in_sysex {
                            sysex_buf.push(byte);
                            if byte == 0xF7 {
                                in_sysex = false;
                                if let Some(frame) = parse_frame(&sysex_buf) {
                                    let _ = tx.send(frame);
                                }
                                sysex_buf.clear();
                            }
                        }
                    }
                }
            },
            (),
        )
        .map_err(|e| e.to_string())?;

    Ok(MidiConnection { output, frame_tx, _input })
}
