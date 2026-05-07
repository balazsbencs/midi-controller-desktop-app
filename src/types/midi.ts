export type MsgType =
  | "None"
  | "Pc"
  | "Cc"
  | "NoteOn"
  | "NoteOff"
  | "ClockBpm"
  | "ClockTap"
  | "Sysex"
  | "Mmc"
  | "Realtime"
  | "Relay"
  | "BankJump"
  | "PageJump"
  | "PresetToggle"
  | "Keystroke";

export type Action =
  | "Press"
  | "Release"
  | "DoubleTap"
  | "LongPress"
  | "LongPressRel"
  | "DoubleTapRel";

export type TogglePos = "Pos1" | "Pos2" | "Shift";

export interface MidiMsg {
  msg_type: MsgType;
  action: Action;
  toggle_pos: TogglePos;
  port: number;
  channel: number;
  number: number;
  value: number;
  sysex: number[];
}

export interface Preset {
  name: string;
  color_pos1: [number, number, number];
  color_pos2: [number, number, number];
  toggle_group: number;
  toggle_reset_group: number;
  msgs: MidiMsg[];
}

export interface Bank {
  name: string;
  presets: [[Preset, Preset, Preset, Preset], [Preset, Preset, Preset, Preset], [Preset, Preset, Preset, Preset], [Preset, Preset, Preset, Preset]];
  expr_presets: [Preset, Preset];
  enter_msgs: MidiMsg[];
  exit_msgs: MidiMsg[];
}

export const MSG_TYPE_LABELS: Record<MsgType, string> = {
  None: "None",
  Pc: "Program Change",
  Cc: "Control Change",
  NoteOn: "Note On",
  NoteOff: "Note Off",
  ClockBpm: "Clock BPM",
  ClockTap: "Tap Tempo",
  Sysex: "SysEx",
  Mmc: "MMC",
  Realtime: "Realtime",
  Relay: "Relay",
  BankJump: "Bank Jump",
  PageJump: "Page Jump",
  PresetToggle: "Preset Toggle",
  Keystroke: "Keystroke",
};

export const ACTION_LABELS: Record<Action, string> = {
  Press: "Press",
  Release: "Release",
  DoubleTap: "Double Tap",
  LongPress: "Long Press",
  LongPressRel: "Long Press Release",
  DoubleTapRel: "Double Tap Release",
};

export const TOGGLE_POS_LABELS: Record<TogglePos, string> = {
  Pos1: "Position 1",
  Pos2: "Position 2",
  Shift: "Shift",
};

export const MIDI_PORT_USB = 0x01;
export const MIDI_PORT_TRS = 0x02;
export const MIDI_PORT_BOTH = 0x03;

export function defaultMidiMsg(): MidiMsg {
  return {
    msg_type: "Pc",
    action: "Press",
    toggle_pos: "Pos1",
    port: MIDI_PORT_USB,
    channel: 0,
    number: 0,
    value: 0,
    sysex: [],
  };
}

export function defaultPreset(name = ""): Preset {
  return {
    name,
    color_pos1: [0, 120, 255],
    color_pos2: [255, 80, 0],
    toggle_group: 0,
    toggle_reset_group: 0,
    msgs: [],
  };
}
