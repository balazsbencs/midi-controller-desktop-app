import { Bank } from "@/types/midi";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Badge } from "@/components/ui/badge";
import { cn } from "@/lib/utils";

interface Props {
  banks: Bank[];
  activeBankIndex: number | null;
  onSelectBank: (index: number) => void;
}

export function BankList({ banks, activeBankIndex, onSelectBank }: Props) {
  return (
    <div className="flex flex-col h-full border-r border-border">
      <div className="px-3 py-2 border-b border-border">
        <span className="text-xs font-semibold text-muted-foreground uppercase tracking-wide">
          Banks
        </span>
      </div>
      <ScrollArea className="flex-1">
        {banks.map((bank, i) => (
          <button
            key={i}
            onClick={() => onSelectBank(i)}
            className={cn(
              "w-full flex items-center gap-2 px-3 py-2 text-sm text-left hover:bg-accent transition-colors",
              activeBankIndex === i && "bg-accent font-medium"
            )}
          >
            <Badge variant="outline" className="text-xs w-7 justify-center shrink-0">
              {i + 1}
            </Badge>
            <span className="truncate">{bank.name || `Bank ${i + 1}`}</span>
          </button>
        ))}
        {banks.length === 0 && (
          <p className="text-xs text-muted-foreground p-3">No banks loaded</p>
        )}
      </ScrollArea>
    </div>
  );
}
