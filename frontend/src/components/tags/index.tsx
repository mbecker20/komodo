import { useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { Badge } from "@ui/badge";
import { Checkbox } from "@ui/checkbox";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
} from "@ui/command";
import { Popover, PopoverContent, PopoverTrigger } from "@ui/popover";
import { Pen } from "lucide-react";
import { useEffect, useState } from "react";

type TargetExcludingSystem = Exclude<Types.ResourceTarget, { type: "System" }>;

export const ResourceTags = ({ target }: { target: TargetExcludingSystem }) => {
  const { type, id } = target;
  const resource = useRead(`List${type}s`, {}).data?.find((d) => d.id === id);
  const tags = useRead("ListTags", {}).data;

  console.log(resource);
  return (
    <>
      {resource?.tags.map((id) => (
        <Badge>{tags?.find((t) => t._id?.$oid === id)?.name}</Badge>
      ))}
    </>
  );
};

export function ManageTags({ target }: { target: TargetExcludingSystem }) {
  const resource = useRead(`List${target.type}s`, {}).data?.find(
    (d) => d.id === target.id
  );

  const [open, setOpen] = useState(false);
  const [input, setInput] = useState("");
  const [tags, setTags] = useState(resource?.tags ?? []);

  const all_tags = useRead("ListTags", {}).data;

  const { mutate: update } = useWrite("UpdateTagsOnResource");
  useEffect(() => {
    update({ target, tags });
  }, [target, tags, update]);

  const { mutateAsync: create } = useWrite("CreateTag");

  const update_tags = (tag: Types.CustomTag) => {
    const included = tags.some((id) => id === tag._id?.$oid);
    if (included) return setTags((t) => t.filter((id) => id !== tag._id?.$oid));
    else return setTags((t) => [...t, tag._id?.$oid as string]);
  };

  if (!resource) return null;

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger disabled={!tags}>
        <Badge className="flex gap-2">
          Edit Tags <Pen className="w-3" />
        </Badge>
      </PopoverTrigger>
      <PopoverContent className="w-[200px] p-0">
        <Command>
          <CommandInput
            placeholder="Search Tags"
            className="h-9"
            value={input}
            onValueChange={setInput}
          />
          <CommandEmpty
            onClick={async () =>
              !!input && update_tags(await create({ name: input }))
            }
          >
            Create Tag
          </CommandEmpty>
          <CommandGroup>
            {all_tags?.map((tag) => (
              <CommandItem
                key={tag._id?.$oid}
                value={tag._id?.$oid}
                onSelect={() => update_tags(tag)}
                className="flex items-center justify-between"
              >
                {tag.name}
                <Checkbox checked={tags.some((id) => id === tag._id?.$oid)} />
              </CommandItem>
            ))}
          </CommandGroup>
        </Command>
      </PopoverContent>
    </Popover>
  );
}
