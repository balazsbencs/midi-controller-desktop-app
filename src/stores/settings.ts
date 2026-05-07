import { create } from "zustand";
import { DeviceSettings, defaultSettings } from "@/types/device";

interface SettingsState {
  settings: DeviceSettings;
  isDirty: boolean;
  setSettings: (settings: DeviceSettings) => void;
  updateSettings: (patch: Partial<DeviceSettings>) => void;
  markClean: () => void;
}

export const useSettingsStore = create<SettingsState>((set) => ({
  settings: defaultSettings(),
  isDirty: false,
  setSettings: (settings) => set({ settings, isDirty: false }),
  updateSettings: (patch) =>
    set((state) => ({
      settings: { ...state.settings, ...patch },
      isDirty: true,
    })),
  markClean: () => set({ isDirty: false }),
}));
