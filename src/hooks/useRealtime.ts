import { useEffect } from "react";
import { listen } from "@tauri-apps/api/event";
import { useBanksStore } from "@/stores/banks";

export function useRealtime() {
  const { setActiveBankIndex, setActivePage } = useBanksStore();

  useEffect(() => {
    const unlistenBankChange = listen<{
      profile: number;
      bank_index: number;
      page: number;
    }>("midi://rt-bank-change", (event) => {
      setActiveBankIndex(event.payload.bank_index);
      setActivePage(event.payload.page);
    });

    return () => {
      unlistenBankChange.then((fn) => fn());
    };
  }, [setActiveBankIndex, setActivePage]);
}
