import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { save, open } from "@tauri-apps/plugin-dialog";
import { writeFile, readFile } from "@tauri-apps/plugin-fs";
import { Button } from "@/components/ui/button";
import { Progress } from "@/components/ui/progress";
import { Alert, AlertDescription } from "@/components/ui/alert";
import { listen } from "@tauri-apps/api/event";

type Status = "idle" | "busy" | "success" | "error";

export function BackupRestore() {
  const [status, setStatus] = useState<Status>("idle");
  const [message, setMessage] = useState("");
  const [progress, setProgress] = useState(0);

  async function handleBackup() {
    setStatus("busy");
    setProgress(0);
    setMessage("Backing up device…");
    try {
      const data = await invoke<number[]>("backup_device");
      const path = await save({
        defaultPath: `daisy-backup-${new Date().toISOString().slice(0, 10)}.daisybak`,
        filters: [{ name: "Daisy Backup", extensions: ["daisybak"] }],
      });
      if (!path) {
        setStatus("idle");
        return;
      }
      await writeFile(path, new Uint8Array(data));
      setStatus("success");
      setMessage("Backup saved successfully.");
    } catch (e) {
      setStatus("error");
      setMessage(String(e));
    }
  }

  async function handleRestore() {
    const path = await open({
      filters: [{ name: "Daisy Backup", extensions: ["daisybak"] }],
    });
    if (!path || Array.isArray(path)) return;

    setStatus("busy");
    setProgress(0);
    setMessage("Restoring…");

    const unlisten = await listen<{ done: number; total: number }>(
      "midi://backup-progress",
      (e) => setProgress(Math.round((e.payload.done / e.payload.total) * 100))
    );

    try {
      const bytes = await readFile(path);
      await invoke("restore_device", { data: Array.from(bytes) });
      setStatus("success");
      setMessage("Restore completed successfully.");
    } catch (e) {
      setStatus("error");
      setMessage(String(e));
    } finally {
      unlisten();
    }
  }

  return (
    <div className="flex flex-col gap-6 max-w-lg">
      <div className="flex flex-col gap-2">
        <h3 className="text-sm font-semibold">Backup</h3>
        <p className="text-sm text-muted-foreground">
          Save all banks, presets, and settings to a file on your computer.
        </p>
        <Button
          variant="outline"
          onClick={handleBackup}
          disabled={status === "busy"}
          className="w-40"
        >
          Export Backup
        </Button>
      </div>

      <div className="flex flex-col gap-2">
        <h3 className="text-sm font-semibold">Restore</h3>
        <p className="text-sm text-muted-foreground">
          Load a previously saved backup file onto the device. This will overwrite all current data.
        </p>
        <Button
          variant="destructive"
          onClick={handleRestore}
          disabled={status === "busy"}
          className="w-40"
        >
          Import Backup
        </Button>
      </div>

      {status === "busy" && (
        <div className="flex flex-col gap-2">
          <p className="text-sm text-muted-foreground">{message}</p>
          <Progress value={progress} className="w-full" />
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
