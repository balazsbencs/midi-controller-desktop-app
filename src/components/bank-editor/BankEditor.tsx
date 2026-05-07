import { Bank, Preset } from "@/types/midi";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { cn } from "@/lib/utils";

interface Props {
  bank: Bank;
  activePresetIndex: number | null;
  onChange: (bank: Bank) => void;
  onSelectPreset: (index: number | null) => void;
}

function PresetSlot({
  preset,
  index,
  isActive,
  onClick,
}: {
  preset: Preset;
  index: number;
  isActive: boolean;
  onClick: () => void;
}) {
  const [r1, g1, b1] = preset.color_pos1;
  const bgColor = `rgb(${r1},${g1},${b1})`;
  const isDark = r1 * 0.299 + g1 * 0.587 + b1 * 0.114 < 128;

  return (
    <button
      onClick={onClick}
      className={cn(
        "relative aspect-square rounded-lg border-2 transition-all text-xs font-medium p-2 flex flex-col items-center justify-center gap-1",
        isActive
          ? "border-primary ring-2 ring-primary/30"
          : "border-border hover:border-primary/50",
      )}
      style={{ backgroundColor: bgColor }}
    >
      <span
        className={cn(
          "text-center text-[10px] leading-tight break-all",
          isDark ? "text-white" : "text-black",
        )}
      >
        {preset.name || `SW ${index + 1}`}
      </span>
      <span
        className={cn("text-[9px]", isDark ? "text-white/60" : "text-black/60")}
      >
        {preset.msgs.length} msg{preset.msgs.length !== 1 ? "s" : ""}
      </span>
    </button>
  );
}

export function BankEditor({
  bank,
  activePresetIndex,
  onChange,
  onSelectPreset,
}: Props) {
  function handleSelectPreset(page: number, sw: number) {
    const globalIndex = page * 4 + sw;
    onSelectPreset(activePresetIndex === globalIndex ? null : globalIndex);
  }

  return (
    <div className="flex flex-col gap-4">
      <div>
        <Label htmlFor="bank-name" className="text-xs">
          Bank Name
        </Label>
        <Input
          id="bank-name"
          maxLength={16}
          value={bank.name}
          onChange={(e) => onChange({ ...bank, name: e.target.value })}
          className="h-8 max-w-64"
        />
      </div>

      <Tabs defaultValue="0">
        <TabsList>
          {[0, 1, 2, 3].map((page) => (
            <TabsTrigger key={page} value={String(page)}>
              Page {page + 1}
            </TabsTrigger>
          ))}
        </TabsList>

        {[0, 1, 2, 3].map((page) => (
          <TabsContent key={page} value={String(page)}>
            <div className="grid grid-cols-4 gap-3 mt-3">
              {[0, 1, 2, 3].map((sw) => (
                <PresetSlot
                  key={sw}
                  preset={bank.presets[page][sw]}
                  index={sw}
                  isActive={activePresetIndex === page * 4 + sw}
                  onClick={() => handleSelectPreset(page, sw)}
                />
              ))}
            </div>
          </TabsContent>
        ))}
      </Tabs>

      <div className="border-t border-border pt-3">
        <p className="text-xs font-semibold text-muted-foreground mb-2">
          Expression Pedals
        </p>
        <div className="grid grid-cols-2 gap-3">
          {[0, 1].map((i) => (
            <div
              key={i}
              className="border border-border rounded-md p-2 text-xs"
            >
              <span className="font-medium">Expr {i + 1}: </span>
              <span className="text-muted-foreground">
                {bank.expr_presets[i].name || "(unnamed)"}
              </span>
              <span className="text-muted-foreground ml-1">
                ({bank.expr_presets[i].msgs.length} msgs)
              </span>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
