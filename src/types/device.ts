export interface DeviceSettings {
  brightness: number;
  startup_bank: number;
  startup_profile: number;
  midi_thru_usb: number;
  midi_thru_trs: number;
  relay_contact: [number, number];
  recall_toggle: boolean;
  device_name: string;
}

export interface DeviceInfo {
  firmware_major: number;
  firmware_minor: number;
  firmware_patch: number;
  device_name: string;
}

export function defaultSettings(): DeviceSettings {
  return {
    brightness: 80,
    startup_bank: 0,
    startup_profile: 0,
    midi_thru_usb: 0x00,
    midi_thru_trs: 0x00,
    relay_contact: [0, 0],
    recall_toggle: false,
    device_name: "Daisy MIDI",
  };
}
