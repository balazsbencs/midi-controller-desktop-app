import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { listen } from "@tauri-apps/api/event";
import { Button } from "@/components/ui/button";
import { Progress } from "@/components/ui/progress";
import { Alert, AlertDescription } from "@/components/ui/alert";

type Status = "idle" | "busy" | "success" | "error";

export function FirmwareUpdate() {
  const [status, setStatus] = useState<Status>("idle");
  const [message, setMessage] = useState("");
  const [progress, setProgress] = useState(0);
  const [firmwarePath, setFirmwarePath] = useState<string | null>(null);

  async function selectFile() {
    const path = await open({
      filters: [{ name: "Firmware", extensions: ["bin"] }],
    });
    if (!path || Array.isArray(path)) return;
    setFirmwarePath(path);
    setStatus("idle");
    setMessage("");
  }

  async function handleFlash() {
    if (!firmwarePath) return;
    setStatus("busy");
    setProgress(0);
    setMessage("Flashing firmware…");

    const unlisten = await listen<{ done: number; total: number }>(
      "midi://firmware-progress",
      (e) => setProgress(Math.round((e.payload.done / e.payload.total) * 100))
    );

    try {
      await invoke("firmware_update", { path: firmwarePath });
      setStatus("success");
      setMessage("Firmware flashed. Device is rebooting.");
    } catch (e) {
      setStatus("error");
      setMessage(String(e));
    } finally {
      unlisten();
    }
  }

  return (
    <div className="flex flex-col gap-6 max-w-lg">
      <Alert>
        <AlertDescription>
          Only flash firmware built for this device. Incorrect firmware may require DFU recovery.
          Hold SW1 + SW2 on power-up to enter ROM bootloader mode if needed.
        </AlertDescription>
      </Alert>

      <div className="flex items-center gap-3">
        <Button variant="outline" onClick={selectFile} disabled={status === "busy"}>
          Choose Firmware (.bin)
        </Button>
        {firmwarePath && (
          <span className="text-sm text-muted-foreground truncate max-w-64">
            {firmwarePath.split("/").pop()}
          </span>
        )}
      </div>

      {firmwarePath && (
        <Button
          onClick={handleFlash}
          disabled={status === "busy"}
          className="w-40"
        >
          {status === "busy" ? "Flashing…" : "Flash Firmware"}
        </Button>
      )}

      {status === "busy" && (
        <div className="flex flex-col gap-2">
          <p className="text-sm text-muted-foreground">{message}</p>
          <Progress value={progress} className="w-full" />
          <p className="text-xs text-muted-foreground">{progress}%</p>
        </div>
      )}

      {(status === "success" || status === "error") && (
        <Alert variant={status === "error" ? "destructive" : "default"}>
          <AlertDescription>{message}</AlertDescription>
        </Alert>
      )}
    </div>
  );
}
