import { create } from "zustand";
import { Bank } from "@/types/midi";

interface BanksState {
  banks: Bank[];
  activeProfile: number;
  activeBankIndex: number | null;
  activePageIndex: number;
  activePresetIndex: number | null;
  setBanks: (banks: Bank[]) => void;
  updateBank: (index: number, bank: Bank) => void;
  setActiveProfile: (profile: number) => void;
  setActiveBankIndex: (index: number | null) => void;
  setActivePage: (page: number) => void;
  setActivePreset: (index: number | null) => void;
  clearBanks: () => void;
}

export const useBanksStore = create<BanksState>((set) => ({
  banks: [],
  activeProfile: 0,
  activeBankIndex: null,
  activePageIndex: 0,
  activePresetIndex: null,
  setBanks: (banks) => set({ banks }),
  updateBank: (index, bank) =>
    set((state) => ({
      banks: state.banks.map((b, i) => (i === index ? bank : b)),
    })),
  setActiveProfile: (activeProfile) => set({ activeProfile, activeBankIndex: null }),
  setActiveBankIndex: (activeBankIndex) => set({ activeBankIndex, activePageIndex: 0, activePresetIndex: null }),
  setActivePage: (activePageIndex) => set({ activePageIndex, activePresetIndex: null }),
  setActivePreset: (activePresetIndex) => set({ activePresetIndex }),
  clearBanks: () =>
    set({ banks: [], activeBankIndex: null, activePresetIndex: null }),
}));
