import { useInvalidate, useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { Badge } from "@ui/badge";
import { Button } from "@ui/button";
import { Checkbox } from "@ui/checkbox";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
} from "@ui/command";
import { DropdownMenu, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuTrigger } from "@ui/dropdown-menu";
import { Popover, PopoverContent, PopoverTrigger } from "@ui/popover";
import { useToast } from "@ui/use-toast";
import { atom, useAtom } from "jotai";
import { Pen, PlusCircle } from "lucide-react";
import { useEffect, useState } from "react";

type TargetExcludingSystem = Exclude<Types.ResourceTarget, { type: "System" }>;

const tagsAtom = atom<string[]>([]);

export const useTagsFilter = () => {
  const [tags, _] = useAtom(tagsAtom);
  return tags
}

export const TagsFilter = () => {
  const [tags, setTags] = useAtom(tagsAtom);
  const all_tags = useRead("ListTags", {}).data;
  return (
    <div className="flex gap-4">
      <TagsWithBadge
        className="cursor-pointer"
        tag_ids={tags}
        onBadgeClick={(tag_id) => setTags(tags.filter((id) => id !== tag_id))}
      />
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button variant="outline" className="flex gap-2">
            Filter by Tag <PlusCircle className="w-4 h-4" />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent className="w-36" side="bottom">
          <DropdownMenuGroup>
            {all_tags
              ?.filter((tag) => !tags.includes(tag._id!.$oid))
              .map((tag) => (
                <DropdownMenuItem
                  className="cursor-pointer"
                  key={tag.name}
                  onClick={() => setTags([...tags, tag._id!.$oid])}
                >
                  {tag.name}
                </DropdownMenuItem>
              ))}
          </DropdownMenuGroup>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
}

export const ResourceTags = ({ target }: { target: TargetExcludingSystem }) => {
  const { type, id } = target;
  const resource = useRead(`List${type}s`, {}).data?.find((d) => d.id === id);

  return <TagsWithBadge tag_ids={resource?.tags} />;
};

export const TagsWithBadge = ({
  tag_ids,
  onBadgeClick,
  className,
}: {
  tag_ids?: string[];
  onBadgeClick?: (tag_id: string) => void;
  className?: string;
}) => {
  const all_tags = useRead("ListTags", {}).data;
  return (
    <>
      {tag_ids?.map((tag_id) => (
        <Badge
          key={tag_id}
          variant="secondary"
          className={className ?? "px-1.5 py-0.5 cursor-pointer"}
          onClick={() => onBadgeClick && onBadgeClick(tag_id)}
        >
          {all_tags?.find((t) => t._id?.$oid === tag_id)?.name ?? "unknown"}
        </Badge>
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
