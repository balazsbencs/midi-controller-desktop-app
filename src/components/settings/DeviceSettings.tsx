import { DeviceSettings } from "@/types/device";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Slider } from "@/components/ui/slider";
import { Switch } from "@/components/ui/switch";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { MIDI_PORT_USB, MIDI_PORT_TRS, MIDI_PORT_BOTH } from "@/types/midi";

interface Props {
  settings: DeviceSettings;
  onChange: (patch: Partial<DeviceSettings>) => void;
}

const PORT_OPTIONS = [
  { value: "0", label: "Off" },
  { value: String(MIDI_PORT_USB), label: "USB" },
  { value: String(MIDI_PORT_TRS), label: "TRS" },
  { value: String(MIDI_PORT_BOTH), label: "Both" },
];

const CONTACT_OPTIONS = [
  { value: "0", label: "Normally Open (NO)" },
  { value: "1", label: "Normally Closed (NC)" },
];

function FormRow({ label, children }: { label: string; children: React.ReactNode }) {
  return (
    <div className="grid grid-cols-[200px_1fr] items-center gap-4">
      <Label className="text-sm">{label}</Label>
      {children}
    </div>
  );
}

export function DeviceSettingsPanel({ settings, onChange }: Props) {
  return (
    <div className="flex flex-col gap-5 max-w-xl">
      <FormRow label="Device Name">
        <Input
          maxLength={20}
          value={settings.device_name}
          onChange={(e) => onChange({ device_name: e.target.value })}
          className="h-8"
        />
      </FormRow>

      <FormRow label={`Brightness: ${settings.brightness}%`}>
        <Slider
          min={30}
          max={100}
          step={1}
          value={[settings.brightness]}
          onValueChange={(v) => {
            const val = Array.isArray(v) ? v[0] : (v as number);
            onChange({ brightness: val });
          }}
          className="w-48"
        />
      </FormRow>

      <FormRow label="Startup Profile">
        <Input
          type="number"
          min={1}
          max={8}
          value={settings.startup_profile + 1}
          onChange={(e) => onChange({ startup_profile: Math.max(0, Number(e.target.value) - 1) })}
          className="h-8 w-20"
        />
      </FormRow>

      <FormRow label="Startup Bank">
        <Input
          type="number"
          min={1}
          max={128}
          value={settings.startup_bank + 1}
          onChange={(e) => onChange({ startup_bank: Math.max(0, Number(e.target.value) - 1) })}
          className="h-8 w-20"
        />
      </FormRow>

      <FormRow label="MIDI Thru (USB In)">
        <Select
          value={String(settings.midi_thru_usb)}
          onValueChange={(v) => v !== null && onChange({ midi_thru_usb: Number(v) })}
        >
          <SelectTrigger className="w-40 h-8">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {PORT_OPTIONS.map((o) => (
              <SelectItem key={o.value} value={o.value}>
                {o.label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </FormRow>

      <FormRow label="MIDI Thru (TRS In)">
        <Select
          value={String(settings.midi_thru_trs)}
          onValueChange={(v) => v !== null && onChange({ midi_thru_trs: Number(v) })}
        >
          <SelectTrigger className="w-40 h-8">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {PORT_OPTIONS.map((o) => (
              <SelectItem key={o.value} value={o.value}>
                {o.label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </FormRow>

      <FormRow label="Relay 1 Contact">
        <Select
          value={String(settings.relay_contact[0])}
          onValueChange={(v) =>
            v !== null &&
            onChange({ relay_contact: [Number(v), settings.relay_contact[1]] })
          }
        >
          <SelectTrigger className="w-48 h-8">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {CONTACT_OPTIONS.map((o) => (
              <SelectItem key={o.value} value={o.value}>
                {o.label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </FormRow>

      <FormRow label="Relay 2 Contact">
        <Select
          value={String(settings.relay_contact[1])}
          onValueChange={(v) =>
            v !== null &&
            onChange({ relay_contact: [settings.relay_contact[0], Number(v)] })
          }
        >
          <SelectTrigger className="w-48 h-8">
            <SelectValue />
          </SelectTrigger>
          <SelectContent>
            {CONTACT_OPTIONS.map((o) => (
              <SelectItem key={o.value} value={o.value}>
                {o.label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </FormRow>

      <FormRow label="Recall Toggle States">
        <Switch
          checked={settings.recall_toggle}
          onCheckedChange={(v) => onChange({ recall_toggle: v })}
        />
      </FormRow>
    </div>
  );
}
