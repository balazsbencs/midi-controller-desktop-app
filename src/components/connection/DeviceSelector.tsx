import { useState } from "react";
import { Button } from "@/components/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";

interface Props {
  ports: string[];
  onConnect: (port: string) => Promise<void>;
  onRefresh: () => Promise<void>;
  loading: boolean;
}

export function DeviceSelector({
  ports,
  onConnect,
  onRefresh,
  loading,
}: Props) {
  const [selected, setSelected] = useState<string>("");

  return (
    <div className="flex items-center gap-3">
      <Select value={selected} onValueChange={(v) => setSelected(v ?? "")}>
        <SelectTrigger className="w-72">
          <SelectValue placeholder="Select MIDI device…" />
        </SelectTrigger>
        <SelectContent>
          {ports.length === 0 ? (
            <SelectItem value="__none__" disabled>
              No MIDI devices found
            </SelectItem>
          ) : (
            ports.map((port) => (
              <SelectItem key={port} value={port}>
                {port}
              </SelectItem>
            ))
          )}
        </SelectContent>
      </Select>

      <Button variant="outline" onClick={onRefresh} disabled={loading}>
        Refresh
      </Button>

      <Button
        onClick={() => onConnect(selected)}
        disabled={!selected || selected === "__none__" || loading}
      >
        {loading ? "Connecting…" : "Connect"}
      </Button>
    </div>
  );
}
