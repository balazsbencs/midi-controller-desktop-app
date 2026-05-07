import { Preset, MidiMsg, defaultMidiMsg } from "@/types/midi";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import { MessageRow } from "./MessageRow";

interface Props {
  preset: Preset;
  onChange: (preset: Preset) => void;
}

function ColorSwatch({ color, label, onChange }: {
  color: [number, number, number];
  label: string;
  onChange: (c: [number, number, number]) => void;
}) {
  const hex = `#${color.map((c) => c.toString(16).padStart(2, "0")).join("")}`;
  return (
    <div className="flex flex-col gap-1">
      <Label className="text-xs">{label}</Label>
      <div className="flex items-center gap-2">
        <div
          className="w-6 h-6 rounded border border-border"
          style={{ backgroundColor: hex }}
        />
        <input
          type="color"
          value={hex}
          className="h-7 w-20 cursor-pointer rounded border border-input px-1"
          onChange={(e) => {
            const v = e.target.value;
            const r = parseInt(v.slice(1, 3), 16);
            const g = parseInt(v.slice(3, 5), 16);
            const b = parseInt(v.slice(5, 7), 16);
            onChange([r, g, b]);
          }}
        />
      </div>
    </div>
  );
}

export function PresetEditor({ preset, onChange }: Props) {
  function update(patch: Partial<Preset>) {
    onChange({ ...preset, ...patch });
  }

  function updateMsg(index: number, msg: MidiMsg) {
    const msgs = preset.msgs.map((m, i) => (i === index ? msg : m));
    update({ msgs });
  }

  function removeMsg(index: number) {
    update({ msgs: preset.msgs.filter((_, i) => i !== index) });
  }

  function addMsg() {
    if (preset.msgs.length >= 32) return;
    update({ msgs: [...preset.msgs, defaultMidiMsg()] });
  }

  return (
    <div className="flex flex-col gap-4 h-full">
      <div className="grid grid-cols-2 gap-3">
        <div>
          <Label htmlFor="preset-name" className="text-xs">Name</Label>
          <Input
            id="preset-name"
            maxLength={16}
            value={preset.name}
            onChange={(e) => update({ name: e.target.value })}
            className="h-8"
          />
        </div>

        <div className="flex gap-3">
          <ColorSwatch
            label="Color Pos 1"
            color={preset.color_pos1}
            onChange={(c) => update({ color_pos1: c })}
          />
          <ColorSwatch
            label="Color Pos 2"
            color={preset.color_pos2}
            onChange={(c) => update({ color_pos2: c })}
          />
        </div>

        <div>
          <Label className="text-xs">Toggle Group (0 = none)</Label>
          <Input
            type="number"
            min={0}
            max={8}
            value={preset.toggle_group}
            onChange={(e) => update({ toggle_group: Number(e.target.value) })}
            className="h-8"
          />
        </div>

        <div>
          <Label className="text-xs">Reset Group (0 = none)</Label>
          <Input
            type="number"
            min={0}
            max={8}
            value={preset.toggle_reset_group}
            onChange={(e) => update({ toggle_reset_group: Number(e.target.value) })}
            className="h-8"
          />
        </div>
      </div>

      <div className="flex items-center justify-between">
        <span className="text-sm font-medium">
          Messages ({preset.msgs.length} / 32)
        </span>
        <Button size="sm" variant="outline" onClick={addMsg} disabled={preset.msgs.length >= 32}>
          + Add Message
        </Button>
      </div>

      <div className="flex text-xs text-muted-foreground gap-2 px-6">
        <span className="w-36">Type</span>
        <span className="w-28">Action</span>
        <span className="w-24">Toggle Pos</span>
        <span className="w-16">Port</span>
        <span className="w-14">Ch</span>
        <span className="w-14">Num</span>
        <span className="w-14">Val</span>
      </div>

      <ScrollArea className="flex-1">
        {preset.msgs.length === 0 ? (
          <p className="text-sm text-muted-foreground py-4 text-center">
            No messages yet. Add one above.
          </p>
        ) : (
          preset.msgs.map((msg, i) => (
            <MessageRow
              key={i}
              msg={msg}
              index={i}
              onChange={updateMsg}
              onRemove={removeMsg}
            />
          ))
        )}
      </ScrollArea>
    </div>
  );
}
