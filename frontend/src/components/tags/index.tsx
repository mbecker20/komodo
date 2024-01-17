import { useInvalidate, useRead, useWrite } from "@lib/hooks";
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
import { useToast } from "@ui/use-toast";
import { Pen, PlusCircle } from "lucide-react";
import { useEffect, useState } from "react";

type TargetExcludingSystem = Exclude<Types.ResourceTarget, { type: "System" }>;

export const ResourceTags = ({ target }: { target: TargetExcludingSystem }) => {
  const { type, id } = target;
  const resource = useRead(`List${type}s`, {}).data?.find((d) => d.id === id);
  const tags = useRead("ListTags", {}).data;

  return (
    <>
      {resource?.tags.map((id) => (
        <Badge key={id}>{tags?.find((t) => t._id?.$oid === id)?.name}</Badge>
      ))}
    </>
  );
};

export const ManageTags = ({ target }: { target: TargetExcludingSystem }) => {
  const { type, id } = target;
  const resource = useRead(`List${type}s`, {}).data?.find((d) => d.id === id);

  const [open, setOpen] = useState(false);
  const [input, setInput] = useState("");
  const [tags, setTags] = useState(resource?.tags);

  const all_tags = useRead("ListTags", {}).data;

  const inv = useInvalidate();
  const { mutate: update } = useWrite("UpdateTagsOnResource", {
    onSuccess: () => inv([`List${type}s`]),
  });

  const { mutateAsync: create } = useWrite("CreateTag", {
    onSuccess: () => inv([`ListTags`]),
  });

  useEffect(() => {
    if (!open && !!resource && !!tags) update({ target, tags });
  }, [target, open, resource, tags, update]);

  useEffect(() => {
    if (resource && !tags) setTags(resource.tags);
  }, [resource, tags]);

  useEffect(() => {
    if (open) setInput("");
  }, [open]);

  const update_tags = (tag: Types.CustomTag) => {
    const exists = tags?.some((id) => id === tag._id?.$oid);
    if (exists) return setTags((t) => t?.filter((id) => id !== tag._id?.$oid));
    else return setTags((t) => [...(t ?? []), tag._id?.$oid as string]);
  };

  const { toast } = useToast();
  const create_tag = async () => {
    if (!input) return toast({ title: "Must provide tag name in input" });
    update_tags(await create({ name: input }));
    setOpen(false);
  };

  if (!resource) return null;

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger>
        <Badge className="flex gap-2" variant="outline">
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
            className="justify-between cursor-pointer hover:bg-accent m-1"
            onClick={create_tag}
          >
            Create Tag
            <PlusCircle className="w-4" />
          </CommandEmpty>
          <CommandGroup>
            {all_tags?.map((tag) => (
              <CommandItem
                key={tag._id?.$oid}
                value={tag.name}
                onSelect={() => update_tags(tag)}
                className="flex items-center justify-between"
              >
                {tag.name}
                <Checkbox checked={tags?.includes(tag._id?.$oid as string)} />
              </CommandItem>
            ))}
          </CommandGroup>
        </Command>
      </PopoverContent>
    </Popover>
  );
};
