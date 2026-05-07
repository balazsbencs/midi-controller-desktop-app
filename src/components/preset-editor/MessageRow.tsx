import {
  MidiMsg,
  Action,
  TogglePos,
  MsgType,
  MSG_TYPE_LABELS,
  ACTION_LABELS,
  TOGGLE_POS_LABELS,
  MIDI_PORT_USB,
  MIDI_PORT_TRS,
  MIDI_PORT_BOTH,
} from "@/types/midi";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";

interface Props {
  msg: MidiMsg;
  index: number;
  onChange: (index: number, msg: MidiMsg) => void;
  onRemove: (index: number) => void;
}

const MSG_TYPES = Object.keys(MSG_TYPE_LABELS) as MsgType[];
const ACTIONS = Object.keys(ACTION_LABELS) as Action[];
const TOGGLE_POSITIONS = Object.keys(TOGGLE_POS_LABELS) as TogglePos[];

const PORT_OPTIONS = [
  { value: MIDI_PORT_USB, label: "USB" },
  { value: MIDI_PORT_TRS, label: "TRS" },
  { value: MIDI_PORT_BOTH, label: "Both" },
];

export function MessageRow({ msg, index, onChange, onRemove }: Props) {
  function update(patch: Partial<MidiMsg>) {
    onChange(index, { ...msg, ...patch });
  }

  const showMidi =
    msg.msg_type !== "Sysex" &&
    msg.msg_type !== "Relay" &&
    msg.msg_type !== "BankJump";

  return (
    <div className="flex items-center gap-2 py-1.5 border-b border-border/50 text-sm">
      <span className="w-5 text-muted-foreground shrink-0">{index + 1}</span>

      <Select
        value={msg.msg_type}
        onValueChange={(v) => v !== null && update({ msg_type: v as MsgType })}
      >
        <SelectTrigger className="w-36 h-7 text-xs">
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          {MSG_TYPES.map((t) => (
            <SelectItem key={t} value={t} className="text-xs">
              {MSG_TYPE_LABELS[t]}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>

      <Select
        value={msg.action}
        onValueChange={(v) => v !== null && update({ action: v as Action })}
      >
        <SelectTrigger className="w-28 h-7 text-xs">
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          {ACTIONS.map((a) => (
            <SelectItem key={a} value={a} className="text-xs">
              {ACTION_LABELS[a]}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>

      <Select
        value={msg.toggle_pos}
        onValueChange={(v) => v !== null && update({ toggle_pos: v as TogglePos })}
      >
        <SelectTrigger className="w-24 h-7 text-xs">
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          {TOGGLE_POSITIONS.map((p) => (
            <SelectItem key={p} value={p} className="text-xs">
              {TOGGLE_POS_LABELS[p]}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>

      {showMidi && (
        <>
          <Select
            value={String(msg.port)}
            onValueChange={(v) => v !== null && update({ port: Number(v) })}
          >
            <SelectTrigger className="w-16 h-7 text-xs">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              {PORT_OPTIONS.map((o) => (
                <SelectItem key={o.value} value={String(o.value)} className="text-xs">
                  {o.label}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>

          <Input
            type="number"
            className="w-14 h-7 text-xs"
            min={1}
            max={16}
            value={msg.channel + 1}
            onChange={(e) => update({ channel: Math.max(0, Number(e.target.value) - 1) })}
            title="MIDI Channel"
          />

          <Input
            type="number"
            className="w-14 h-7 text-xs"
            min={0}
            max={127}
            value={msg.number}
            onChange={(e) => update({ number: Number(e.target.value) })}
            title="Number (CC/PC/Note)"
          />

          <Input
            type="number"
            className="w-14 h-7 text-xs"
            min={0}
            max={127}
            value={msg.value}
            onChange={(e) => update({ value: Number(e.target.value) })}
            title="Value"
          />
        </>
      )}

      {msg.msg_type === "Sysex" && (
        <Input
          className="flex-1 h-7 text-xs font-mono"
          placeholder="F0 … F7"
          value={msg.sysex
            .map((b) => b.toString(16).padStart(2, "0").toUpperCase())
            .join(" ")}
          onChange={(e) => {
            const bytes = e.target.value
              .split(/\s+/)
              .filter(Boolean)
              .map((h) => parseInt(h, 16))
              .filter((n) => !isNaN(n));
            update({ sysex: bytes });
          }}
        />
      )}

      <Button
        variant="ghost"
        size="icon"
        className="h-7 w-7 shrink-0 text-destructive hover:text-destructive"
        onClick={() => onRemove(index)}
      >
        ×
      </Button>
    </div>
  );
}
