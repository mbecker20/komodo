import { useRead } from "@lib/hooks";
import { Button } from "@ui/button";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@ui/command";
import { Popover, PopoverContent, PopoverTrigger } from "@ui/popover";
import { PlusCircle, SearchX } from "lucide-react";
import { useState } from "react";

export const AddExtraArgMenu = ({
  onSelect,
}: {
  onSelect: (suggestion: string) => void;
}) => {
  const [open, setOpen] = useState(false);
  const [search, setSearch] = useState("");
  const suggestions = useRead("ListCommonExtraArgs", {}).data;
  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button
          variant="secondary"
          className="flex items-center gap-2 w-[200px]"
        >
          <PlusCircle className="w-4 h-4" /> Add Extra Arg
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-[300px] max-h-[400px] p-0" align="end">
        <Command>
          <CommandInput
            placeholder="Search suggestions"
            className="h-9"
            value={search}
            onValueChange={setSearch}
          />
          <CommandList>
            <CommandEmpty className="flex justify-evenly items-center">
              No Suggestions Found
              <SearchX className="w-3 h-3" />
            </CommandEmpty>

            <CommandGroup>
              <CommandItem
                onSelect={() => {
                  onSelect("");
                  setOpen(false);
                }}
                className="w-full cursor-pointer"
              >
                Empty Extra Arg
              </CommandItem>

              {suggestions?.map((suggestion) => (
                <CommandItem
                  key={suggestion}
                  onSelect={() => {
                    onSelect(suggestion);
                    setOpen(false);
                  }}
                  className="w-full overflow-hidden overflow-ellipsis cursor-pointer"
                >
                  {suggestion}
                </CommandItem>
              ))}
            </CommandGroup>
          </CommandList>
        </Command>
      </PopoverContent>
    </Popover>
  );
};
