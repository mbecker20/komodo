import { useAllResources, useUser } from "@lib/hooks";
import { Button } from "@ui/button";
import {
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandList,
  CommandSeparator,
  CommandItem,
} from "@ui/command";
import { Home, Search, User } from "lucide-react";
import { Fragment, ReactNode, useMemo, useState } from "react";
import { useNavigate } from "react-router-dom";
import { cn, RESOURCE_TARGETS, usableResourcePath } from "@lib/utils";
import { Badge } from "@ui/badge";
import { ResourceComponents } from "./resources";

export const OmniSearch = ({
  className,
  setOpen,
}: {
  className?: string;
  setOpen: (open: boolean) => void;
}) => {
  return (
    <Button
      variant="outline"
      onClick={() => setOpen(true)}
      className={cn(
        "flex items-center gap-4 w-fit md:w-[200px] lg:w-[300px] xl:w-[400px] justify-between hover:bg-card/50",
        className
      )}
    >
      <div className="flex items-center gap-4">
        <Search className="w-4 h-4" />{" "}
        <span className="text-muted-foreground hidden md:flex">Search</span>
      </div>
      <Badge
        variant="outline"
        className="text-muted-foreground hidden md:inline-flex"
      >
        shift + s
      </Badge>
    </Button>
  );
};

type OmniItem = {
  key: string;
  label: string;
  icon: ReactNode;
  onSelect: () => void;
};

export const OmniDialog = ({
  open,
  setOpen,
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
}) => {
  const [search, setSearch] = useState("");
  const navigate = useNavigate();
  const nav = (value: string) => {
    setOpen(false);
    navigate(value);
  };
  const items = useOmniItems(nav, search);
  return (
    <CommandDialog open={open} onOpenChange={setOpen} manualFilter>
      <CommandInput
        placeholder="Search for resources..."
        value={search}
        onValueChange={setSearch}
      />
      <CommandList>
        <CommandEmpty>No results found.</CommandEmpty>

        {Object.entries(items)
          .filter(([_, items]) => items.length > 0)
          .map(([key, items], i) => (
            <Fragment key={key}>
              {i !== 0 && <CommandSeparator />}
              <CommandGroup heading={key ? key : undefined}>
                {items.map(({ key, label, icon, onSelect }) => (
                  <CommandItem
                    key={key}
                    value={key}
                    className="flex items-center gap-2 cursor-pointer"
                    onSelect={onSelect}
                  >
                    {icon}
                    {label}
                  </CommandItem>
                ))}
              </CommandGroup>
            </Fragment>
          ))}
      </CommandList>
    </CommandDialog>
  );
};

const useOmniItems = (
  nav: (path: string) => void,
  search: string
): Record<string, OmniItem[]> => {
  const user = useUser().data;
  const resources = useAllResources();
  const searchTerms = search
    .toLowerCase()
    .split(" ")
    .filter((term) => term);
  return useMemo(
    () => ({
      "": [
        {
          key: "Home",
          label: "Home",
          icon: <Home className="w-4 h-4" />,
          onSelect: () => nav("/"),
        },
        ...RESOURCE_TARGETS.map((_type) => {
          const type =
            _type === "ResourceSync"
              ? "Sync"
              : _type === "ServerTemplate"
                ? "Template"
                : _type;
          const Components = ResourceComponents[_type];
          return {
            key: type + "s",
            label: type + "s",
            icon: <Components.Icon />,
            onSelect: () => nav(usableResourcePath(_type)),
          };
        }),
        (user?.admin && {
          key: "Users",
          label: "Users",
          icon: <User className="w-4 h-4" />,
          onSelect: () => nav("/users"),
        }) as OmniItem,
      ]
        .filter((item) => item)
        .filter((item) => {
          const label = item.label.toLowerCase();
          return (
            searchTerms.length === 0 ||
            searchTerms.every((term) => label.includes(term))
          );
        }),
      ...Object.fromEntries(
        RESOURCE_TARGETS.map((_type) => {
          const type =
            _type === "ResourceSync"
              ? "Sync"
              : _type === "ServerTemplate"
                ? "Template"
                : _type;
          const lower = type.toLowerCase();
          const Components = ResourceComponents[_type];
          return [
            type + "s",
            resources[_type]
              ?.filter(
                (item) =>
                  searchTerms.length === 0 ||
                  searchTerms.every(
                    (term) =>
                      item.name.toLowerCase().includes(term) ||
                      lower.includes(term)
                  )
              )
              .map((server) => ({
                key: type + "-" + server.name,
                label: server.name,
                icon: <Components.Icon id={server.id} />,
                onSelect: () =>
                  nav(`/${usableResourcePath(_type)}/${server.id}`),
              })) || [],
          ];
        })
      ),
    }),
    [user, resources, search]
  );
};
