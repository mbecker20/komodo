import { useState } from "react";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/ui/select";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
} from "@ui/command";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@ui/popover";
import { Button } from "@ui/button";
import { Check, ChevronDown } from "lucide-react";
import { cn } from "@util/helpers";

export type SelectorItem = {
  value: string;
  label: string;
};

export function Selector({
  value = { value: "unknown", label: "default value" },
  items = [],
  onSelect,
  placeholder,
  width,
  disabled,
}: {
  value?: SelectorItem;
  items?: SelectorItem[];
  onSelect?: (item: SelectorItem) => void;
  placeholder?: string;
  width?: string;
  disabled?: boolean;
}) {
  return (
    <Select
      value={value?.value || undefined}
      onValueChange={(v) => {
        const item = items.find(({ value }) => value === v);
        if (item === undefined) {
          console.error("could not find selected item. selected:", v);
          return;
        }
        onSelect && onSelect(item);
      }}
      disabled={disabled}
    >
      <SelectTrigger className={width && `w-[${width}]`}>
        <SelectValue placeholder={placeholder} />
      </SelectTrigger>
      <SelectContent>
        <SelectGroup>
          {items.map(({ value, label }) => (
            <SelectItem key={value} value={value}>
              {label}
            </SelectItem>
          ))}
        </SelectGroup>
      </SelectContent>
    </Select>
  );
}

export function SearchableSelector({
  value = { value: "unknown", label: "default value" },
  items = [],
  onSelect,
  placeholder,
  width = "200px",
  disabled,
}: {
  value?: SelectorItem;
  items?: SelectorItem[];
  onSelect?: (value: string) => void;
  placeholder?: string;
  width?: string;
  disabled?: boolean;
}) {
  const [open, setOpen] = useState(false);
  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button
          variant="outline"
          role="combobox"
          aria-expanded={open}
          className={`w-[${width}] justify-between`}
          disabled={disabled}
        >
          {value && value.label ? value.label : placeholder}
          <ChevronDown className="ml-2 h-4 w-4 shrink-0 opacity-50" />
        </Button>
      </PopoverTrigger>
      <PopoverContent className={`w-[${width}] p-0`}>
        <Command>
          <CommandInput placeholder="Search" />
          <CommandEmpty>No items found.</CommandEmpty>
          <CommandGroup>
            {items.map((item) => (
              <CommandItem
                key={item.value}
                onSelect={() => {
                  onSelect && onSelect(item.value);
                  setOpen(false);
                }}
              >
                <Check
                  className={cn(
                    "mr-2 h-4 w-4",
                    value.value === item.value ? "opacity-100" : "opacity-0"
                  )}
                />
                {item.label}
              </CommandItem>
            ))}
          </CommandGroup>
        </Command>
      </PopoverContent>
    </Popover>
  );
}
