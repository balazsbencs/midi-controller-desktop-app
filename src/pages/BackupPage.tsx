import { BackupRestore } from "@/components/backup/BackupRestore";

export function BackupPage() {
  return (
    <div className="flex flex-col gap-6 p-6">
      <h2 className="text-base font-semibold">Backup & Restore</h2>
      <BackupRestore />
    </div>
  );
}
