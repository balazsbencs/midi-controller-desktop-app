import { HashRouter, Routes, Route, NavLink, Navigate } from "react-router-dom";
import { Moon, Sun } from "lucide-react";
import { Separator } from "@/components/ui/separator";
import { Button } from "@/components/ui/button";
import { ConnectPage } from "@/pages/ConnectPage";
import { EditorPage } from "@/pages/EditorPage";
import { SettingsPage } from "@/pages/SettingsPage";
import { BackupPage } from "@/pages/BackupPage";
import { FirmwarePage } from "@/pages/FirmwarePage";
import { useConnectionStore } from "@/stores/connection";
import { useTheme } from "@/hooks/useTheme";

function navClass({ isActive }: { isActive: boolean }) {
  return `text-xs px-3 py-1.5 rounded transition-colors ${
    isActive
      ? "bg-primary text-primary-foreground"
      : "text-muted-foreground hover:text-foreground hover:bg-accent"
  }`;
}

export function ThemeToggle({ className }: { className?: string }) {
  const { theme, toggle } = useTheme();
  return (
    <Button
      variant="ghost"
      size="icon"
      className={`h-6 w-6 ${className ?? ""}`}
      onClick={toggle}
      title="Toggle theme"
    >
      {theme === "dark" ? <Sun className="h-3.5 w-3.5" /> : <Moon className="h-3.5 w-3.5" />}
    </Button>
  );
}

function AppShell({ children }: { children: React.ReactNode }) {
  const { isConnected, portName } = useConnectionStore();

  return (
    <div className="flex flex-col h-screen">
      {isConnected && (
        <nav className="flex items-center gap-1 px-4 py-1.5 border-b border-border bg-muted/40 shrink-0">
          <NavLink to="/editor" className={navClass}>
            Editor
          </NavLink>
          <NavLink to="/settings" className={navClass}>
            Settings
          </NavLink>
          <NavLink to="/backup" className={navClass}>
            Backup
          </NavLink>
          <NavLink to="/firmware" className={navClass}>
            Firmware
          </NavLink>
          <Separator orientation="vertical" className="h-4 mx-1" />
          <span className="text-xs text-muted-foreground">{portName}</span>
          <div className="ml-auto">
            <ThemeToggle />
          </div>
        </nav>
      )}
      <div className="flex-1 overflow-auto">{children}</div>
    </div>
  );
}

export default function App() {
  const { isConnected } = useConnectionStore();

  return (
    <HashRouter>
      <Routes>
        <Route path="/" element={<ConnectPage />} />
        <Route
          path="/editor"
          element={isConnected ? <EditorPage /> : <Navigate to="/" replace />}
        />
        <Route
          path="/settings"
          element={
            isConnected ? (
              <AppShell>
                <SettingsPage />
              </AppShell>
            ) : (
              <Navigate to="/" replace />
            )
          }
        />
        <Route
          path="/backup"
          element={
            isConnected ? (
              <AppShell>
                <BackupPage />
              </AppShell>
            ) : (
              <Navigate to="/" replace />
            )
          }
        />
        <Route
          path="/firmware"
          element={
            isConnected ? (
              <AppShell>
                <FirmwarePage />
              </AppShell>
            ) : (
              <Navigate to="/" replace />
            )
          }
        />
        <Route path="*" element={<Navigate to="/" replace />} />
      </Routes>
    </HashRouter>
  );
}
