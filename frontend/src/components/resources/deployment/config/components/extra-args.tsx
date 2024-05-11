import { ConfigItem } from "@components/config/util";
import { useRead } from "@lib/hooks";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@ui/command";
import { Input } from "@ui/input";
import { Popover, PopoverContent, PopoverTrigger } from "@ui/popover";
import { MinusCircle, PlusCircle, SearchX } from "lucide-react";
import { useState } from "react";

export const ExtraArgs = ({
  args,
  set,
  disabled,
}: {
  args: string[];
  set: (update: Partial<Types.DeploymentConfig>) => void;
  disabled: boolean;
}) => {
  return (
    <ConfigItem
      label="Extra Args"
      className={args.length > 0 ? "items-start" : undefined}
    >
      <div className="flex flex-col gap-4 w-full max-w-[400px]">
        {args.map((arg, i) => (
          <div className="w-full flex gap-4" key={i}>
            <Input
              value={arg}
              placeholder="--extra-arg=value"
              onChange={(e) => {
                args[i] = e.target.value;
                set({ extra_args: [...args] });
              }}
              disabled={disabled}
            />
            {!disabled && (
              <Button
                variant="secondary"
                onClick={() =>
                  set({ extra_args: [...args.filter((_, idx) => idx !== i)] })
                }
              >
                <MinusCircle className="w-4 h-4" />
              </Button>
            )}
          </div>
        ))}

        {!disabled && (
          <div className="place-self-end">
            <AddExtraArgMenu
              onSelect={(suggestion) =>
                set({ extra_args: [...args, suggestion] })
              }
            />
          </div>
        )}
      </div>
    </ConfigItem>
  );
};

const AddExtraArgMenu = ({
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
      <PopoverContent className="w-[300px] max-h-[400px] p-0">
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
                onSelect={() => onSelect("")}
                className="w-full cursor-pointer"
              >
                Empty Extra Arg
              </CommandItem>

              {suggestions?.map((suggestion) => (
                <CommandItem
                  key={suggestion}
                  onSelect={() => onSelect(suggestion)}
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
