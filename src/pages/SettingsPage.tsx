import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Button } from "@/components/ui/button";
import { Alert, AlertDescription } from "@/components/ui/alert";
import { DeviceSettingsPanel } from "@/components/settings/DeviceSettings";
import { useSettingsStore } from "@/stores/settings";

export function SettingsPage() {
  const { settings, updateSettings, isDirty, markClean } = useSettingsStore();
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [success, setSuccess] = useState(false);

  async function handleSave() {
    setSaving(true);
    setError(null);
    setSuccess(false);
    try {
      await invoke("set_settings", { settings });
      markClean();
      setSuccess(true);
      setTimeout(() => setSuccess(false), 2000);
    } catch (e) {
      setError(String(e));
    } finally {
      setSaving(false);
    }
  }

  return (
    <div className="flex flex-col gap-6 p-6">
      <div className="flex items-center justify-between">
        <h2 className="text-base font-semibold">Device Settings</h2>
        <Button onClick={handleSave} disabled={saving || !isDirty} size="sm">
          {saving ? "Saving…" : success ? "Saved!" : "Save to Device"}
        </Button>
      </div>

      {error && (
        <Alert variant="destructive">
          <AlertDescription>{error}</AlertDescription>
        </Alert>
      )}

      <DeviceSettingsPanel settings={settings} onChange={updateSettings} />
    </div>
  );
}
