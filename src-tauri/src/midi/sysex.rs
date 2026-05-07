/// SysEx frame constants — match firmware sysex_builder.h / sysex_commands.h
pub const MFR_0: u8 = 0x00;
pub const MFR_1: u8 = 0x21;
pub const MFR_2: u8 = 0x7F;
pub const DEVICE_ID: u8 = 0x01;

pub const DIR_HOST_TO_DEV: u8 = 0x00;
pub const DIR_DEV_TO_HOST: u8 = 0x01;

/// Command bytes (must match sysex_commands.h)
pub mod cmd {
    pub const GET_DEVICE_INFO: u8 = 0x01;
    pub const DEVICE_INFO_RSP: u8 = 0x02;
    pub const GET_BANK: u8 = 0x10;
    pub const BANK_DATA_RSP: u8 = 0x11;
    pub const SET_BANK: u8 = 0x12;
    pub const SET_BANK_ACK: u8 = 0x13;
    pub const GET_ALL_BANKS: u8 = 0x20;
    pub const ALL_BANKS_CHUNK: u8 = 0x21;
    pub const ALL_BANKS_DONE: u8 = 0x22;
    pub const GET_SETTINGS: u8 = 0x30;
    pub const SETTINGS_RSP: u8 = 0x31;
    pub const SET_SETTINGS: u8 = 0x32;
    pub const BACKUP_REQUEST: u8 = 0x40;
    pub const BACKUP_CHUNK: u8 = 0x41;
    pub const RESTORE_CHUNK: u8 = 0x42;
    pub const RESTORE_DONE: u8 = 0x43;
    pub const ENTER_EDITOR_MODE: u8 = 0x50;
    pub const EXIT_EDITOR_MODE: u8 = 0x51;
    pub const FW_UPDATE_START: u8 = 0x60;
    pub const FW_CHUNK: u8 = 0x61;
    pub const FW_REBOOT: u8 = 0x62;
    pub const RT_BANK_CHANGE: u8 = 0x70;
    pub const RT_PRESET_TOGGLE: u8 = 0x71;
}

/// Minimum frame length: F0 MFR(3) DEV CMD DIR SEQ LEN(2) CHKSUM F7 = 11 bytes
pub const MIN_FRAME_LEN: usize = 11;

#[derive(Debug, Clone)]
pub struct SysexFrame {
    pub cmd: u8,
    #[allow(dead_code)]
    pub dir: u8,
    pub seq: u8,
    pub payload: Vec<u8>,
}

/// Build a complete SysEx frame. Payload must already be 7-bit safe.
pub fn build_frame(cmd: u8, dir: u8, seq: u8, payload: &[u8]) -> Vec<u8> {
    let plen = payload.len();
    let mut out = Vec::with_capacity(MIN_FRAME_LEN + plen);
    out.push(0xF0);
    out.push(MFR_0);
    out.push(MFR_1);
    out.push(MFR_2);
    out.push(DEVICE_ID);
    out.push(cmd);
    out.push(dir);
    out.push(seq & 0x7F);
    out.push(((plen >> 7) & 0x7F) as u8);
    out.push((plen & 0x7F) as u8);
    out.extend_from_slice(payload);
    let chk = compute_checksum(cmd, dir, seq & 0x7F, plen, payload);
    out.push(chk);
    out.push(0xF7);
    out
}

pub fn parse_frame(frame: &[u8]) -> Option<SysexFrame> {
    if frame.len() < MIN_FRAME_LEN {
        return None;
    }
    if frame[0] != 0xF0 || *frame.last()? != 0xF7 {
        return None;
    }
    if frame[1] != MFR_0 || frame[2] != MFR_1 || frame[3] != MFR_2 || frame[4] != DEVICE_ID {
        return None;
    }
    let cmd = frame[5];
    let dir = frame[6];
    let seq = frame[7];
    let plen = (((frame[8] & 0x7F) as usize) << 7) | ((frame[9] & 0x7F) as usize);
    if frame.len() != MIN_FRAME_LEN + plen {
        return None;
    }
    let payload = &frame[10..10 + plen];
    let chk = compute_checksum(cmd, dir, seq, plen, payload);
    if chk != frame[10 + plen] {
        return None;
    }
    Some(SysexFrame { cmd, dir, seq, payload: payload.to_vec() })
}

fn compute_checksum(cmd: u8, dir: u8, seq: u8, plen: usize, payload: &[u8]) -> u8 {
    let mut chk = cmd ^ dir ^ (seq & 0x7F)
        ^ (((plen >> 7) & 0x7F) as u8)
        ^ ((plen & 0x7F) as u8);
    for &b in payload {
        chk ^= b;
    }
    chk & 0x7F
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_empty_payload() {
        let frame = build_frame(cmd::GET_DEVICE_INFO, DIR_HOST_TO_DEV, 0x01, &[]);
        let parsed = parse_frame(&frame).expect("parse failed");
        assert_eq!(parsed.cmd, cmd::GET_DEVICE_INFO);
        assert_eq!(parsed.seq, 0x01);
        assert!(parsed.payload.is_empty());
    }

    #[test]
    fn round_trip_with_payload() {
        let payload = vec![0x01, 0x02, 0x03, 0x7F];
        let frame = build_frame(cmd::GET_BANK, DIR_HOST_TO_DEV, 0x42, &payload);
        let parsed = parse_frame(&frame).unwrap();
        assert_eq!(parsed.payload, payload);
        assert_eq!(parsed.seq, 0x42);
    }

    #[test]
    fn rejects_bad_checksum() {
        let mut frame = build_frame(cmd::GET_DEVICE_INFO, DIR_HOST_TO_DEV, 0x01, &[]);
        let last = frame.len() - 2;
        frame[last] ^= 0x01;
        assert!(parse_frame(&frame).is_none());
    }

    #[test]
    fn all_bytes_are_7bit_safe() {
        let frame = build_frame(cmd::GET_DEVICE_INFO, DIR_HOST_TO_DEV, 0x01, &[]);
        // All bytes between F0 and F7 must be ≤ 0x7F
        for &b in &frame[1..frame.len() - 1] {
            assert!(b <= 0x7F, "non-7bit byte: {b:#04x}");
        }
    }
}
