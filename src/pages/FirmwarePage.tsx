import { FirmwareUpdate } from "@/components/firmware/FirmwareUpdate";

export function FirmwarePage() {
  return (
    <div className="flex flex-col gap-6 p-6">
      <h2 className="text-base font-semibold">Firmware Update</h2>
      <FirmwareUpdate />
    </div>
  );
}
