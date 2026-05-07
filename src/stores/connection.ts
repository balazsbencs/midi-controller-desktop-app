import { create } from "zustand";
import { DeviceInfo } from "@/types/device";

interface ConnectionState {
  isConnected: boolean;
  portName: string | null;
  availablePorts: string[];
  deviceInfo: DeviceInfo | null;
  setConnected: (portName: string, info: DeviceInfo) => void;
  setDisconnected: () => void;
  setPorts: (ports: string[]) => void;
}

export const useConnectionStore = create<ConnectionState>((set) => ({
  isConnected: false,
  portName: null,
  availablePorts: [],
  deviceInfo: null,
  setConnected: (portName, deviceInfo) =>
    set({ isConnected: true, portName, deviceInfo }),
  setDisconnected: () =>
    set({ isConnected: false, portName: null, deviceInfo: null }),
  setPorts: (availablePorts) => set({ availablePorts }),
}));
