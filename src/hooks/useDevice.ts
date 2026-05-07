import { invoke } from "@tauri-apps/api/core";
import { DeviceInfo } from "@/types/device";
import { Bank } from "@/types/midi";
import { useConnectionStore } from "@/stores/connection";
import { useBanksStore } from "@/stores/banks";
import { useSettingsStore } from "@/stores/settings";

export function useDevice() {
  const { setConnected, setDisconnected, setPorts } = useConnectionStore();
  const { setBanks, clearBanks } = useBanksStore();
  const { setSettings } = useSettingsStore();

  async function listPorts(): Promise<string[]> {
    const ports = await invoke<string[]>("list_midi_devices");
    setPorts(ports);
    return ports;
  }

  async function connect(portName: string): Promise<void> {
    await invoke("connect_device", { portName });
    await invoke("enter_editor_mode");
    const info = await invoke<DeviceInfo>("get_device_info");
    setConnected(portName, info);

    const settings = await invoke<ReturnType<typeof useSettingsStore.getState>["settings"]>(
      "get_settings"
    );
    setSettings(settings);
  }

  async function disconnect(): Promise<void> {
    await invoke("disconnect_device");
    setDisconnected();
    clearBanks();
  }

  async function loadAllBanks(profile: number): Promise<void> {
    const banks = await invoke<Bank[]>("get_all_banks", { profile });
    setBanks(banks);
  }

  async function saveBank(profile: number, bankIndex: number, bank: Bank): Promise<void> {
    await invoke("set_bank", { profile, bankIndex, bank });
  }

  return { listPorts, connect, disconnect, loadAllBanks, saveBank };
}
