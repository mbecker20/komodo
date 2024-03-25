import { useInvalidate, useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { Badge } from "@ui/badge";
import { Button } from "@ui/button";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
} from "@ui/command";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@ui/dropdown-menu";
import { Popover, PopoverContent, PopoverTrigger } from "@ui/popover";
import { useToast } from "@ui/use-toast";
import { atom, useAtom } from "jotai";
import { PlusCircle } from "lucide-react";
import { useEffect, useState } from "react";

type TargetExcludingSystem = Exclude<Types.ResourceTarget, { type: "System" }>;

const tagsAtom = atom<string[]>([]);

export const useTagsFilter = () => {
  const [tags, _] = useAtom(tagsAtom);
  return tags;
};

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
};

export const ResourceTags = ({
  target,
  click_to_delete,
}: {
  target: TargetExcludingSystem;
  click_to_delete?: boolean;
}) => {
  const inv = useInvalidate();
  const { type, id } = target;
  const resource = useRead(`List${type}s`, {}).data?.find((d) => d.id === id);
  const { mutate } = useWrite("UpdateTagsOnResource", {
    onSuccess: () => {
      inv([`List${type}s`]);
    },
  });

  return (
    <TagsWithBadge
      tag_ids={resource?.tags}
      onBadgeClick={(tag_id) => {
        if (!click_to_delete) return;
        mutate({
          target,
          tags: resource!.tags.filter((tag) => tag !== tag_id),
        });
      }}
    />
  );
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

export const AddTags = ({ target }: { target: TargetExcludingSystem }) => {
  const { toast } = useToast();

  const { type, id } = target;
  const resource = useRead(`List${type}s`, {}).data?.find((d) => d.id === id);

  const [open, setOpen] = useState(false);
  const [input, setInput] = useState("");

  const all_tags = useRead("ListTags", {}).data;

  const inv = useInvalidate();

  const { mutate: update } = useWrite("UpdateTagsOnResource", {
    onSuccess: () => {
      inv([`List${type}s`]);
      toast({ title: `Added tag ${input}` });
      setOpen(false);
    },
  });

  const { mutateAsync: create } = useWrite("CreateTag", {
    onSuccess: () => inv([`ListTags`]),
  });

  // useEffect(() => {
  //   if (!open && !!resource && !!tags) update({ target, tags });
  // }, [target, open, resource, tags, update]);

  // useEffect(() => {
  //   if (resource && !tags) setTags(resource.tags);
  // }, [resource, tags]);

  useEffect(() => {
    if (open) setInput("");
  }, [open]);

  const create_tag = async () => {
    if (!input) return toast({ title: "Must provide tag name in input" });
    const tag = await create({ name: input });
    update({
      target,
      tags: [...(resource?.tags ?? []), tag._id!.$oid],
    });
    setOpen(false);
  };

  if (!resource) return null;

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button variant="secondary" className="px-2 py-0 h-auto">
          <PlusCircle className="w-3" />
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-[200px] p-0" sideOffset={12}>
        <Command>
          <CommandInput
            placeholder="Search / Create"
            className="h-9"
            value={input}
            onValueChange={setInput}
            onKeyDown={(e) => {
              if (
                e.key === "Enter" &&
                // check that no tags still match
                all_tags?.every((tag) => !tag.name.includes(input))
              ) {
                create_tag();
              }
            }}
          />
          <CommandEmpty
            className="justify-between cursor-pointer hover:bg-accent m-1"
            onClick={create_tag}
          >
            Create Tag
            <PlusCircle className="w-4" />
          </CommandEmpty>
          <CommandGroup>
            {all_tags
              ?.filter((tag) => !resource?.tags.includes(tag._id!.$oid))
              .map((tag) => (
                <CommandItem
                  key={tag._id?.$oid}
                  value={tag.name}
                  onSelect={() =>
                    update({
                      target,
                      tags: [...(resource?.tags ?? []), tag._id!.$oid],
                    })
                  }
                  className="flex items-center justify-between"
                >
                  <div className="p-1">{tag.name}</div>
                </CommandItem>
              ))}
          </CommandGroup>
        </Command>
      </PopoverContent>
    </Popover>
  );
};
