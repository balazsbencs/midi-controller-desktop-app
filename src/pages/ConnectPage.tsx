import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { Moon, Sun } from "lucide-react";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Alert, AlertDescription } from "@/components/ui/alert";
import { Button } from "@/components/ui/button";
import { DeviceSelector } from "@/components/connection/DeviceSelector";
import { useConnectionStore } from "@/stores/connection";
import { useDevice } from "@/hooks/useDevice";
import { useTheme } from "@/hooks/useTheme";

export function ConnectPage() {
  const navigate = useNavigate();
  const { availablePorts } = useConnectionStore();
  const { listPorts, connect } = useDevice();
  const { theme, toggle } = useTheme();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    listPorts().catch(() => {});
  }, []);

  async function handleRefresh() {
    setError(null);
    try {
      await listPorts();
    } catch (e) {
      setError(String(e));
    }
  }

  async function handleConnect(port: string) {
    if (!port) return;
    setLoading(true);
    setError(null);
    try {
      await connect(port);
      navigate("/editor");
    } catch (e) {
      setError(String(e));
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className="min-h-screen flex items-center justify-center bg-background p-6">
      <div className="absolute top-3 right-3">
        <Button variant="ghost" size="icon" className="h-7 w-7" onClick={toggle} title="Toggle theme">
          {theme === "dark" ? <Sun className="h-4 w-4" /> : <Moon className="h-4 w-4" />}
        </Button>
      </div>

      <Card className="w-full max-w-md">
        <CardHeader>
          <CardTitle>Daisy MIDI Editor</CardTitle>
          <CardDescription>
            Connect your pedal via USB, then select it from the list below.
          </CardDescription>
        </CardHeader>
        <CardContent className="flex flex-col gap-4">
          <DeviceSelector
            ports={availablePorts}
            onConnect={handleConnect}
            onRefresh={handleRefresh}
            loading={loading}
          />

          {error && (
            <Alert variant="destructive">
              <AlertDescription>{error}</AlertDescription>
            </Alert>
          )}

          <p className="text-xs text-muted-foreground">
            Make sure the device is connected via USB-C and powered on.
          </p>
        </CardContent>
      </Card>
    </div>
  );
}
