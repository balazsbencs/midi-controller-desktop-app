import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { Separator } from "@/components/ui/separator";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Alert, AlertDescription } from "@/components/ui/alert";
import { BankList } from "@/components/bank-editor/BankList";
import { BankEditor } from "@/components/bank-editor/BankEditor";
import { PresetEditor } from "@/components/preset-editor/PresetEditor";
import { useConnectionStore } from "@/stores/connection";
import { useBanksStore } from "@/stores/banks";
import { useDevice } from "@/hooks/useDevice";
import { useRealtime } from "@/hooks/useRealtime";
import { Bank, Preset } from "@/types/midi";
import { ThemeToggle } from "@/App";

export function EditorPage() {
  const navigate = useNavigate();
  const { isConnected, deviceInfo, portName } = useConnectionStore();
  const {
    banks,
    activeProfile,
    activeBankIndex,
    activePresetIndex,
    setActiveBankIndex,
    setActiveProfile,
    setActivePreset,
    updateBank,
  } = useBanksStore();
  const { loadAllBanks, saveBank, disconnect } = useDevice();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [saving, setSaving] = useState(false);

  useRealtime();

  useEffect(() => {
    if (!isConnected) {
      navigate("/");
      return;
    }
    if (banks.length === 0) {
      handleLoadBanks();
    }
  }, [isConnected]);

  async function handleLoadBanks() {
    setLoading(true);
    setError(null);
    try {
      await loadAllBanks(activeProfile);
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }

  async function handleProfileChange(profile: string | null) {
    if (profile === null) return;
    setActiveProfile(Number(profile));
    setLoading(true);
    setError(null);
    try {
      await loadAllBanks(Number(profile));
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }

  async function handleDisconnect() {
    await disconnect();
    navigate("/");
  }

  function handleBankChange(bank: Bank) {
    if (activeBankIndex === null) return;
    updateBank(activeBankIndex, bank);
  }

  async function handleSaveBank() {
    if (activeBankIndex === null) return;
    setSaving(true);
    setError(null);
    try {
      await saveBank(activeProfile, activeBankIndex, banks[activeBankIndex]);
    } catch (e) {
      setError(String(e));
    } finally {
      setSaving(false);
    }
  }

  function handlePresetChange(preset: Preset) {
    if (activeBankIndex === null || activePresetIndex === null) return;
    const page = Math.floor(activePresetIndex / 4);
    const sw = activePresetIndex % 4;
    const currentBank = banks[activeBankIndex];
    const updatedPresets = currentBank.presets.map((pagePresets, p) =>
      p === page
        ? pagePresets.map((s, i) => (i === sw ? preset : s))
        : pagePresets,
    ) as Bank["presets"];
    updateBank(activeBankIndex, { ...currentBank, presets: updatedPresets });
  }

  const activeBank = activeBankIndex !== null ? banks[activeBankIndex] : null;
  const activePreset =
    activeBank && activePresetIndex !== null
      ? activeBank.presets[Math.floor(activePresetIndex / 4)][
          activePresetIndex % 4
        ]
      : null;

  return (
    <div className="flex flex-col h-screen bg-background">
      {/* Topbar */}
      <header className="flex items-center gap-3 px-4 py-2 border-b border-border shrink-0">
        <span className="font-semibold text-sm">Daisy MIDI Editor</span>
        <Separator orientation="vertical" className="h-4" />
        <Badge variant="secondary" className="text-xs">
          {portName}
        </Badge>
        {deviceInfo && (
          <span className="text-xs text-muted-foreground">
            {deviceInfo.device_name} v{deviceInfo.firmware_major}.
            {deviceInfo.firmware_minor}.{deviceInfo.firmware_patch}
          </span>
        )}
        <div className="ml-auto flex items-center gap-2">
          <Select
            value={String(activeProfile)}
            onValueChange={handleProfileChange}
          >
            <SelectTrigger className="h-7 w-28 text-xs">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              {[0, 1, 2, 3, 4, 5, 6, 7].map((p) => (
                <SelectItem key={p} value={String(p)} className="text-xs">
                  Profile {p + 1}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
          {activeBankIndex !== null && (
            <Button
              size="sm"
              onClick={handleSaveBank}
              disabled={saving}
              className="h-7 text-xs"
            >
              {saving ? "Saving…" : "Save Bank"}
            </Button>
          )}
          <Button
            size="sm"
            variant="outline"
            onClick={handleDisconnect}
            className="h-7 text-xs"
          >
            Disconnect
          </Button>
          <ThemeToggle />
        </div>
      </header>

      {error && (
        <Alert variant="destructive" className="m-2">
          <AlertDescription>{error}</AlertDescription>
        </Alert>
      )}

      {loading ? (
        <div className="flex-1 flex items-center justify-center text-sm text-muted-foreground">
          Loading banks…
        </div>
      ) : (
        <div className="flex flex-1 overflow-hidden">
          {/* Bank list sidebar */}
          <div className="w-48 shrink-0 overflow-hidden">
            <BankList
              banks={banks}
              activeBankIndex={activeBankIndex}
              onSelectBank={setActiveBankIndex}
            />
          </div>

          {/* Main editor area */}
          <div className="flex-1 flex overflow-hidden">
            {activeBank ? (
              <>
                {/* Bank editor (4×4 grid + bank name) */}
                <div className="w-72 shrink-0 p-4 border-r border-border overflow-auto">
                  <BankEditor
                    bank={activeBank}
                    activePresetIndex={activePresetIndex}
                    onChange={handleBankChange}
                    onSelectPreset={setActivePreset}
                  />
                </div>

                {/* Preset editor panel */}
                <div className="flex-1 p-4 overflow-hidden flex flex-col">
                  {activePreset ? (
                    <>
                      <div className="flex items-center justify-between mb-3">
                        <h2 className="text-sm font-semibold">
                          Preset Editor — {activePreset.name || "Untitled"}
                        </h2>
                        <Button
                          size="sm"
                          variant="ghost"
                          onClick={() => setActivePreset(null)}
                          className="h-7 text-xs"
                        >
                          Close
                        </Button>
                      </div>
                      <PresetEditor
                        preset={activePreset}
                        onChange={handlePresetChange}
                      />
                    </>
                  ) : (
                    <div className="flex-1 flex items-center justify-center text-sm text-muted-foreground">
                      Select a switch slot to edit its preset
                    </div>
                  )}
                </div>
              </>
            ) : (
              <div className="flex-1 flex items-center justify-center text-sm text-muted-foreground">
                Select a bank from the sidebar
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
}
