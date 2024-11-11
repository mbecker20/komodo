import {
  useInvalidate,
  useRead,
  useShiftKeyListener,
  useWrite,
} from "@lib/hooks";
import { cn } from "@lib/utils";
import { Badge } from "@ui/badge";
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
import { useToast } from "@ui/use-toast";
import { Types } from "komodo_client";
import { Check, MinusCircle, SearchX, Tag } from "lucide-react";
import { useMemo, useState } from "react";
import { useSearchParams } from "react-router-dom";

type TargetExcludingSystem = Exclude<Types.ResourceTarget, { type: "System" }>;

export const TagSelector = () => {
  const all_tags = useRead("ListTags", {}).data;

  const [params, setParams] = useSearchParams();
  const selected = params.getAll("tag");

  const [open, setOpen] = useState(false);
  useShiftKeyListener("T", () => setOpen(true));
  useShiftKeyListener("C", () => {
    params.delete("tag");
    setParams(params);
  });

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button variant="outline" className="flex items-center gap-2">
          <Tag className="w-3 h-3" />
          Tag Filter
        </Button>
      </PopoverTrigger>
      <PopoverContent
        className="w-[200px] max-h-[200px] p-0"
        sideOffset={12}
        align="end"
      >
        <Command shouldFilter={false}>
          <CommandInput placeholder="Search Tags" className="h-9" />
          <CommandList>
            <CommandEmpty className="flex justify-evenly items-center pt-2">
              No Tags Found
              <SearchX className="w-3 h-3" />
            </CommandEmpty>

            <CommandGroup>
              {all_tags?.map(({ name }) => (
                <CommandItem
                  key={name}
                  className="flex items-center justify-between cursor-pointer"
                  onSelect={() => {
                    if (selected.includes(name)) params.delete("tag", name);
                    else params.append("tag", name);
                    setParams(params);
                  }}
                >
                  <p className="p-1">{name}</p>
                  {selected.includes(name) && <Check className="w-3" />}
                </CommandItem>
              ))}
            </CommandGroup>
          </CommandList>
        </Command>
      </PopoverContent>
    </Popover>
  );
};

export const ResourceTagsV2 = ({
  target,
  className,
  clickHandler,
  //   onClick,
}: {
  target: TargetExcludingSystem;
  className?: string;
  //   onClick?: (tagId: string) => void;
  clickHandler?: "toggle" | "remove";
}) => {
  const all_tags = useRead("ListTags", {}).data;

  const [params, setParams] = useSearchParams();
  const selected = params.getAll("tag");

  const { type, id } = target;
  const resource = useRead(`List${type}s`, {}).data?.find((r) => r.id === id);

  const { toast } = useToast();
  const invalidate = useInvalidate();
  const { mutate } = useWrite("UpdateTagsOnResource", {
    onSuccess: () => invalidate([`List${type}s`]),
  });

  const resourceTags = useMemo(
    () =>
      resource?.tags
        .map((tagId) => {
          const tag = all_tags?.find((t) => t._id?.$oid === tagId);
          if (!tag) return null;
          return {
            tagId: tag._id?.$oid!,
            name: tag.name,
            active: selected.includes(tag?.name),
          };
        })
        .filter((tag) => tag !== null)
        .sort((a, b) => (a.name > b.name ? 1 : -1)),
    [all_tags, resource, selected]
  );

  return (
    <div className="flex gap-1 overflow-x-auto accent-scrollbar">
      {resourceTags?.map(({ tagId, name, active }) => (
        <Badge
          key={tagId}
          className={cn(
            "h-7 p-0 whitespace-nowrap flex items-center justify-between",
            clickHandler && "cursor-pointer ",
            className
          )}
          variant={active ? "default" : "secondary"}
          onClick={(e) => {
            e.preventDefault();
            e.stopPropagation();

            // toggle tags in and out of url params when clicked
            if (clickHandler === "toggle") {
              if (active) params.delete("tag", name);
              else params.append("tag", name);
              setParams(params);
            }

            // remove tag from resource when clicked
            if (clickHandler === "remove") {
              mutate(
                { target, tags: resource?.tags.filter((t) => t !== tagId)! },
                {
                  onSuccess: () =>
                    toast({
                      title: "Tag Removed",
                      description: `Removed tag - ${name} - from ${resource?.name}`,
                    }),
                }
              );
            }
          }}
        >
          <p className="px-2">{name}</p>
          {clickHandler === "remove" && (
            <p className="w-7 h-full grid place-items-center rounded-r-md hover:bg-red-700">
              <MinusCircle className="w-3" />
            </p>
          )}
        </Badge>
      ))}
    </div>
  );
};
