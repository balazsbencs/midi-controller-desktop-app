mod midi;
mod state;

use serde::Serialize;
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;

use midi::connection;
use midi::protocol::Protocol;
use midi::sysex::{self, SysexFrame};
use midi::types::{Bank, DeviceInfo, DeviceSettings};
use state::{AppState, MOCK_DEVICE_NAME};

type State<'a> = tauri::State<'a, Mutex<AppState>>;

// ── Connection commands ──────────────────────────────────────────────────────

#[tauri::command]
fn list_midi_devices() -> Result<Vec<String>, String> {
    let mut ports = connection::list_ports()?;
    ports.push(MOCK_DEVICE_NAME.to_string());
    Ok(ports)
}

#[tauri::command]
async fn connect_device(port_name: String, state: State<'_>, app: AppHandle) -> Result<(), String> {
    if port_name == MOCK_DEVICE_NAME {
        let mut s = state.lock().await;
        s.mock = true;
        s.connection = None;
        s.seq = 0;
        app.emit("midi://device-connected", &port_name).ok();
        return Ok(());
    }

    let conn = connection::connect(&port_name)?;

    let rt_rx = conn.frame_tx.subscribe();
    let app_clone = app.clone();
    tokio::spawn(async move {
        forward_rt_events(rt_rx, app_clone).await;
    });

    {
        let mut s = state.lock().await;
        s.mock = false;
        s.connection = Some(conn);
        s.seq = 0;
    }

    app.emit("midi://device-connected", &port_name).ok();
    Ok(())
}

#[tauri::command]
async fn disconnect_device(state: State<'_>, app: AppHandle) -> Result<(), String> {
    let mut s = state.lock().await;
    if s.mock {
        s.mock = false;
        drop(s);
        app.emit("midi://device-disconnected", ()).ok();
        return Ok(());
    }
    if let Some(mut conn) = s.connection.take() {
        let _ = conn.output.send(&sysex::build_frame(
            sysex::cmd::EXIT_EDITOR_MODE,
            sysex::DIR_HOST_TO_DEV,
            0,
            &[],
        ));
    }
    drop(s);
    app.emit("midi://device-disconnected", ()).ok();
    Ok(())
}

// ── Session commands ─────────────────────────────────────────────────────────

#[tauri::command]
async fn enter_editor_mode(state: State<'_>) -> Result<(), String> {
    let mut s = state.lock().await;
    if s.mock {
        return Ok(());
    }
    let (output, frame_tx, seq) = s.protocol_parts()?;
    Protocol::new(output, &frame_tx, seq).enter_editor_mode().await
}

#[tauri::command]
async fn exit_editor_mode(state: State<'_>) -> Result<(), String> {
    let mut s = state.lock().await;
    if s.mock {
        return Ok(());
    }
    let (output, frame_tx, seq) = s.protocol_parts()?;
    Protocol::new(output, &frame_tx, seq).exit_editor_mode().await
}

#[tauri::command]
async fn get_device_info(state: State<'_>) -> Result<DeviceInfo, String> {
    let mut s = state.lock().await;
    if s.mock {
        return Ok(DeviceInfo {
            firmware_major: 1,
            firmware_minor: 0,
            firmware_patch: 0,
            device_name: "Daisy Mock".to_string(),
        });
    }
    let (output, frame_tx, seq) = s.protocol_parts()?;
    Protocol::new(output, &frame_tx, seq).get_device_info().await
}

// ── Settings commands ────────────────────────────────────────────────────────

#[tauri::command]
async fn get_settings(state: State<'_>) -> Result<DeviceSettings, String> {
    let mut s = state.lock().await;
    if s.mock {
        return Ok(s.mock_settings.clone());
    }
    let (output, frame_tx, seq) = s.protocol_parts()?;
    Protocol::new(output, &frame_tx, seq).get_settings().await
}

#[tauri::command]
async fn set_settings(settings: DeviceSettings, state: State<'_>) -> Result<(), String> {
    let mut s = state.lock().await;
    if s.mock {
        s.mock_settings = settings;
        return Ok(());
    }
    let (output, frame_tx, seq) = s.protocol_parts()?;
    Protocol::new(output, &frame_tx, seq).set_settings(&settings).await
}

// ── Bank commands ────────────────────────────────────────────────────────────

#[tauri::command]
async fn get_bank(profile: u8, bank_index: u8, state: State<'_>) -> Result<Bank, String> {
    let mut s = state.lock().await;
    if s.mock {
        return s
            .mock_banks
            .get(profile as usize)
            .and_then(|banks| banks.get(bank_index as usize))
            .cloned()
            .ok_or_else(|| format!("Mock bank {bank_index} not found in profile {profile}"));
    }
    let (output, frame_tx, seq) = s.protocol_parts()?;
    Protocol::new(output, &frame_tx, seq).get_bank(profile, bank_index).await
}

#[tauri::command]
async fn set_bank(profile: u8, bank_index: u8, bank: Bank, state: State<'_>) -> Result<(), String> {
    let mut s = state.lock().await;
    if s.mock {
        if let Some(banks) = s.mock_banks.get_mut(profile as usize) {
            if let Some(slot) = banks.get_mut(bank_index as usize) {
                *slot = bank;
            }
        }
        return Ok(());
    }
    let (output, frame_tx, seq) = s.protocol_parts()?;
    Protocol::new(output, &frame_tx, seq).set_bank(profile, bank_index, &bank).await
}

#[tauri::command]
async fn get_all_banks(profile: u8, state: State<'_>) -> Result<Vec<Bank>, String> {
    let mut s = state.lock().await;
    if s.mock {
        return Ok(s.mock_banks.get(profile as usize).cloned().unwrap_or_default());
    }
    let (output, frame_tx, seq) = s.protocol_parts()?;
    Protocol::new(output, &frame_tx, seq).get_all_banks(profile).await
}

// ── Backup / Restore ─────────────────────────────────────────────────────────

#[tauri::command]
async fn backup_device(state: State<'_>) -> Result<Vec<u8>, String> {
    let mut s = state.lock().await;
    if s.mock {
        return Ok(b"DAISY_MOCK_BACKUP_V1".to_vec());
    }
    let (output, frame_tx, seq) = s.protocol_parts()?;
    Protocol::new(output, &frame_tx, seq).backup_device().await
}

#[tauri::command]
async fn restore_device(data: Vec<u8>, state: State<'_>, app: AppHandle) -> Result<(), String> {
    let mut s = state.lock().await;
    if s.mock {
        app.emit("midi://backup-progress", ProgressEvent { done: 1, total: 1 }).ok();
        return Ok(());
    }
    let (output, frame_tx, seq) = s.protocol_parts()?;
    Protocol::new(output, &frame_tx, seq)
        .restore_device(&data, |done, total| {
            app.emit("midi://backup-progress", ProgressEvent { done, total }).ok();
        })
        .await
}

// ── Firmware update ──────────────────────────────────────────────────────────

#[tauri::command]
async fn firmware_update(path: String, state: State<'_>, app: AppHandle) -> Result<(), String> {
    let mut s = state.lock().await;
    if s.mock {
        app.emit("midi://firmware-progress", ProgressEvent { done: 1, total: 1 }).ok();
        return Ok(());
    }
    let firmware = std::fs::read(&path).map_err(|e| e.to_string())?;
    let crc = crc32_ieee(&firmware);
    let (output, frame_tx, seq) = s.protocol_parts()?;
    Protocol::new(output, &frame_tx, seq)
        .firmware_update(&firmware, crc, |done, total| {
            app.emit("midi://firmware-progress", ProgressEvent { done, total }).ok();
        })
        .await
}

// ── Helpers ──────────────────────────────────────────────────────────────────

#[derive(Serialize, Clone)]
struct ProgressEvent {
    done: usize,
    total: usize,
}

async fn forward_rt_events(
    mut rx: tokio::sync::broadcast::Receiver<SysexFrame>,
    app: AppHandle,
) {
    use sysex::cmd;
    loop {
        match rx.recv().await {
            Ok(frame) => match frame.cmd {
                cmd::RT_BANK_CHANGE if frame.payload.len() >= 3 => {
                    app.emit(
                        "midi://rt-bank-change",
                        serde_json::json!({
                            "profile": frame.payload[0],
                            "bank_index": frame.payload[1],
                            "page": frame.payload[2],
                        }),
                    )
                    .ok();
                }
                cmd::RT_PRESET_TOGGLE if frame.payload.len() >= 2 => {
                    app.emit(
                        "midi://rt-preset-toggle",
                        serde_json::json!({
                            "switch_index": frame.payload[0],
                            "toggle_pos": frame.payload[1],
                        }),
                    )
                    .ok();
                }
                _ => {}
            },
            Err(tokio::sync::broadcast::error::RecvError::Lagged(_)) => continue,
            Err(_) => break,
        }
    }
}

fn crc32_ieee(data: &[u8]) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            crc = if crc & 1 != 0 { (crc >> 1) ^ 0xEDB8_8320 } else { crc >> 1 };
        }
    }
    !crc
}

// ── Tauri entry point ────────────────────────────────────────────────────────

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .manage(Mutex::new(AppState::new()))
        .invoke_handler(tauri::generate_handler![
            list_midi_devices,
            connect_device,
            disconnect_device,
            enter_editor_mode,
            exit_editor_mode,
            get_device_info,
            get_settings,
            set_settings,
            get_bank,
            set_bank,
            get_all_banks,
            backup_device,
            restore_device,
            firmware_update,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
