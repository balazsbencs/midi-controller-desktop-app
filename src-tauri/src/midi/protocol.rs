use std::time::Duration;
use tokio::sync::broadcast;
use tokio::time::timeout;

use super::codec::{decode_7bit, encode_7bit};
use super::sysex::{build_frame, cmd, SysexFrame, DIR_HOST_TO_DEV};
use super::types::{
    deserialise_bank, deserialise_settings, serialise_bank, serialise_settings, Bank, DeviceInfo,
    DeviceSettings,
};

const RESPONSE_TIMEOUT: Duration = Duration::from_secs(5);
const CHUNK_TIMEOUT: Duration = Duration::from_secs(10);

pub struct Protocol<'a> {
    output: &'a mut midir::MidiOutputConnection,
    frame_rx: broadcast::Receiver<SysexFrame>,
    seq: &'a mut u8,
}

impl<'a> Protocol<'a> {
    pub fn new(
        output: &'a mut midir::MidiOutputConnection,
        frame_tx: &broadcast::Sender<SysexFrame>,
        seq: &'a mut u8,
    ) -> Self {
        Self { output, frame_rx: frame_tx.subscribe(), seq }
    }

    fn next_seq(&mut self) -> u8 {
        let s = *self.seq;
        *self.seq = (*self.seq + 1) & 0x7F;
        s
    }

    fn send(&mut self, frame: Vec<u8>) -> Result<(), String> {
        self.output.send(&frame).map_err(|e| e.to_string())
    }

    /// Wait for the next frame matching the given command and sequence number.
    async fn await_response(&mut self, expected_cmd: u8, expected_seq: u8) -> Result<SysexFrame, String> {
        let deadline = timeout(RESPONSE_TIMEOUT, async {
            loop {
                match self.frame_rx.recv().await {
                    Ok(f) if f.cmd == expected_cmd && f.seq == expected_seq => return Ok(f),
                    Ok(_) => continue,
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(_) => return Err("MIDI channel closed".to_string()),
                }
            }
        });
        deadline.await.map_err(|_| "Timeout waiting for device response".to_string())?
    }

    pub async fn enter_editor_mode(&mut self) -> Result<(), String> {
        let seq = self.next_seq();
        let frame = build_frame(cmd::ENTER_EDITOR_MODE, DIR_HOST_TO_DEV, seq, &[]);
        self.send(frame)
    }

    pub async fn exit_editor_mode(&mut self) -> Result<(), String> {
        let seq = self.next_seq();
        let frame = build_frame(cmd::EXIT_EDITOR_MODE, DIR_HOST_TO_DEV, seq, &[]);
        self.send(frame)
    }

    pub async fn get_device_info(&mut self) -> Result<DeviceInfo, String> {
        let seq = self.next_seq();
        let frame = build_frame(cmd::GET_DEVICE_INFO, DIR_HOST_TO_DEV, seq, &[]);
        self.send(frame)?;
        let resp = self.await_response(cmd::DEVICE_INFO_RSP, seq).await?;
        let decoded = decode_7bit(&resp.payload);
        if decoded.len() < 3 {
            return Err("Device info response too short".to_string());
        }
        let firmware_major = decoded[0];
        let firmware_minor = decoded[1];
        let firmware_patch = decoded[2];
        let device_name = cstr_from_bytes(&decoded[3..]);
        Ok(DeviceInfo { firmware_major, firmware_minor, firmware_patch, device_name })
    }

    pub async fn get_settings(&mut self) -> Result<DeviceSettings, String> {
        let seq = self.next_seq();
        let frame = build_frame(cmd::GET_SETTINGS, DIR_HOST_TO_DEV, seq, &[]);
        self.send(frame)?;
        let resp = self.await_response(cmd::SETTINGS_RSP, seq).await?;
        let decoded = decode_7bit(&resp.payload);
        if decoded.len() < 29 {
            return Err(format!("Settings response too short: {} bytes", decoded.len()));
        }
        Ok(deserialise_settings(&decoded))
    }

    pub async fn set_settings(&mut self, settings: &DeviceSettings) -> Result<(), String> {
        let raw = serialise_settings(settings);
        let encoded = encode_7bit(&raw);
        let seq = self.next_seq();
        let frame = build_frame(cmd::SET_SETTINGS, DIR_HOST_TO_DEV, seq, &encoded);
        self.send(frame)
    }

    pub async fn get_bank(&mut self, profile: u8, bank_index: u8) -> Result<Bank, String> {
        let seq = self.next_seq();
        let payload = [profile & 0x7F, bank_index & 0x7F];
        let frame = build_frame(cmd::GET_BANK, DIR_HOST_TO_DEV, seq, &payload);
        self.send(frame)?;
        let resp = self.await_response(cmd::BANK_DATA_RSP, seq).await?;
        let decoded = decode_7bit(&resp.payload);
        if decoded.len() < super::types::BANK_SIZE {
            return Err(format!(
                "Bank response too short: {} bytes (expected {})",
                decoded.len(),
                super::types::BANK_SIZE
            ));
        }
        Ok(deserialise_bank(&decoded))
    }

    pub async fn set_bank(&mut self, profile: u8, bank_index: u8, bank: &Bank) -> Result<(), String> {
        let mut raw = vec![profile & 0x7F, bank_index & 0x7F];
        raw.extend_from_slice(&serialise_bank(bank));
        let encoded = encode_7bit(&raw);
        let seq = self.next_seq();
        let frame = build_frame(cmd::SET_BANK, DIR_HOST_TO_DEV, seq, &encoded);
        self.send(frame)?;
        let resp = self.await_response(cmd::SET_BANK_ACK, seq).await?;
        if resp.payload.len() >= 2 && resp.payload[1] != 0 {
            return Err(format!("Device rejected bank write: status={}", resp.payload[1]));
        }
        Ok(())
    }

    pub async fn get_all_banks(&mut self, profile: u8) -> Result<Vec<Bank>, String> {
        let seq = self.next_seq();
        let frame = build_frame(cmd::GET_ALL_BANKS, DIR_HOST_TO_DEV, seq, &[profile & 0x7F]);
        self.send(frame)?;

        let mut chunks: Vec<(usize, Vec<u8>)> = Vec::new();
        let mut total_chunks = 0usize;

        loop {
            let resp = timeout(CHUNK_TIMEOUT, async {
                loop {
                    match self.frame_rx.recv().await {
                        Ok(f)
                            if f.cmd == cmd::ALL_BANKS_CHUNK || f.cmd == cmd::ALL_BANKS_DONE =>
                        {
                            return Ok(f);
                        }
                        Ok(_) => continue,
                        Err(broadcast::error::RecvError::Lagged(_)) => continue,
                        Err(_) => return Err("channel closed"),
                    }
                }
            })
            .await
            .map_err(|_| "Timeout during get_all_banks".to_string())?
            .map_err(|e| e.to_string())?;

            if resp.cmd == cmd::ALL_BANKS_DONE {
                break;
            }
            // ALL_BANKS_CHUNK: [chunk_index][total_chunks][data…]
            if resp.payload.len() < 2 {
                continue;
            }
            let chunk_idx = resp.payload[0] as usize;
            total_chunks = resp.payload[1] as usize;
            let data = decode_7bit(&resp.payload[2..]);
            chunks.push((chunk_idx, data));
        }

        // Assemble chunks in order
        chunks.sort_by_key(|(i, _)| *i);
        let raw: Vec<u8> = chunks.into_iter().flat_map(|(_, d)| d).collect();

        let bank_size = super::types::BANK_SIZE;
        let num_banks = if total_chunks > 0 { 128 } else { raw.len() / bank_size };
        let banks = (0..num_banks)
            .map(|i| {
                let off = i * bank_size;
                if off + bank_size <= raw.len() {
                    deserialise_bank(&raw[off..])
                } else {
                    Bank {
                        name: format!("Bank {}", i + 1),
                        presets: std::array::from_fn(|_| std::array::from_fn(|_| Default::default())),
                        expr_presets: std::array::from_fn(|_| Default::default()),
                        enter_msgs: vec![],
                        exit_msgs: vec![],
                    }
                }
            })
            .collect();

        Ok(banks)
    }

    pub async fn backup_device(&mut self) -> Result<Vec<u8>, String> {
        let seq = self.next_seq();
        let frame = build_frame(cmd::BACKUP_REQUEST, DIR_HOST_TO_DEV, seq, &[]);
        self.send(frame)?;

        let mut chunks: Vec<(usize, Vec<u8>)> = Vec::new();

        loop {
            let resp = timeout(CHUNK_TIMEOUT, async {
                loop {
                    match self.frame_rx.recv().await {
                        Ok(f)
                            if f.cmd == cmd::BACKUP_CHUNK =>
                        {
                            return Ok(f);
                        }
                        Ok(_) => continue,
                        Err(broadcast::error::RecvError::Lagged(_)) => continue,
                        Err(_) => return Err("channel closed"),
                    }
                }
            })
            .await
            .map_err(|_| "Timeout during backup".to_string())?
            .map_err(|e| e.to_string())?;

            if resp.payload.len() < 2 {
                continue;
            }
            let chunk_idx = resp.payload[0] as usize;
            let total = resp.payload[1] as usize;
            let data = decode_7bit(&resp.payload[2..]);
            chunks.push((chunk_idx, data));
            if chunk_idx + 1 >= total {
                break;
            }
        }

        chunks.sort_by_key(|(i, _)| *i);
        Ok(chunks.into_iter().flat_map(|(_, d)| d).collect())
    }

    pub async fn restore_device(
        &mut self,
        data: &[u8],
        progress_cb: impl Fn(usize, usize),
    ) -> Result<(), String> {
        const CHUNK_BYTES: usize = 512;
        let total = (data.len() + CHUNK_BYTES - 1) / CHUNK_BYTES;

        for (i, chunk) in data.chunks(CHUNK_BYTES).enumerate() {
            let mut payload = vec![i as u8, total as u8];
            payload.extend_from_slice(chunk);
            let encoded = encode_7bit(&payload);
            let seq = self.next_seq();
            let frame = build_frame(cmd::RESTORE_CHUNK, DIR_HOST_TO_DEV, seq, &encoded);
            self.send(frame)?;
            progress_cb(i + 1, total);
        }

        let seq = self.next_seq();
        let frame = build_frame(cmd::RESTORE_DONE, DIR_HOST_TO_DEV, seq, &[]);
        self.send(frame)
    }

    pub async fn firmware_update(
        &mut self,
        firmware: &[u8],
        crc32: u32,
        progress_cb: impl Fn(usize, usize),
    ) -> Result<(), String> {
        // FW_UPDATE_START: [size_3..0][crc_3..0] (8 bytes, 7-bit encoded)
        let size = firmware.len() as u32;
        let mut start_payload = Vec::with_capacity(8);
        start_payload.extend_from_slice(&size.to_be_bytes());
        start_payload.extend_from_slice(&crc32.to_be_bytes());
        let encoded_start = encode_7bit(&start_payload);
        let seq = self.next_seq();
        let frame = build_frame(cmd::FW_UPDATE_START, DIR_HOST_TO_DEV, seq, &encoded_start);
        self.send(frame)?;
        // Wait for ACK (any response to this SEQ)
        let _ack = self.await_response(cmd::FW_UPDATE_START, seq).await?;

        const CHUNK_BYTES: usize = 512;
        let total = (firmware.len() + CHUNK_BYTES - 1) / CHUNK_BYTES;
        for (i, chunk) in firmware.chunks(CHUNK_BYTES).enumerate() {
            let encoded = encode_7bit(chunk);
            let seq = self.next_seq();
            let frame = build_frame(cmd::FW_CHUNK, DIR_HOST_TO_DEV, seq, &encoded);
            self.send(frame)?;
            progress_cb(i + 1, total);
        }

        let seq = self.next_seq();
        let frame = build_frame(cmd::FW_REBOOT, DIR_HOST_TO_DEV, seq, &[]);
        self.send(frame)
    }
}

fn cstr_from_bytes(buf: &[u8]) -> String {
    let end = buf.iter().position(|&b| b == 0).unwrap_or(buf.len());
    String::from_utf8_lossy(&buf[..end]).into_owned()
}
