import { useRead, useUser } from "@lib/hooks";
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
import { Home, Search, UserCircle2 } from "lucide-react";
import { Fragment } from "react";
import { useNavigate } from "react-router-dom";
import { ResourceComponents } from "./resources";
import { UsableResource } from "@types";
import { RESOURCE_TARGETS, cn, usableResourcePath } from "@lib/utils";
import { DeploymentComponents } from "./resources/deployment";
import { BuildComponents } from "./resources/build";
import { ServerComponents } from "./resources/server";
import { ProcedureComponents } from "./resources/procedure";
import { RepoComponents } from "./resources/repo";

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
        "flex items-center gap-4 w-fit md:w-[200px] lg:w-[300px] justify-start",
        className
      )}
    >
      <Search className="w-4 h-4" />{" "}
      <span className="text-muted-foreground hidden md:flex">
        Search {"(shift+s)"}
      </span>
    </Button>
  );
};

export const OmniDialog = ({
  open,
  setOpen,
}: {
  open: boolean;
  setOpen: (open: boolean) => void;
}) => {
  const user = useUser().data;
  const navigate = useNavigate();
  const nav = (value: string) => {
    setOpen(false);
    navigate(value);
  };
  return (
    <CommandDialog open={open} onOpenChange={setOpen}>
      <CommandInput placeholder="Type a command or search..." />
      <CommandList>
        <CommandEmpty>No results found.</CommandEmpty>

        <CommandGroup>
          <CommandItem
            className="flex items-center gap-2 cursor-pointer"
            onSelect={() => nav("/")}
          >
            <Home className="w-4 h-4" />
            Home
          </CommandItem>
          <CommandItem
            className="flex items-center gap-2 cursor-pointer"
            onSelect={() => nav("/servers")}
          >
            <ServerComponents.Icon />
            Servers
          </CommandItem>
          <CommandItem
            className="flex items-center gap-2 cursor-pointer"
            onSelect={() => nav("/deployments")}
          >
            <DeploymentComponents.Icon />
            Deployments
          </CommandItem>
          <CommandItem
            className="flex items-center gap-2 cursor-pointer"
            onSelect={() => nav("/builds")}
          >
            <BuildComponents.Icon />
            Builds
          </CommandItem>
          <CommandItem
            className="flex items-center gap-2 cursor-pointer"
            onSelect={() => nav("/repos")}
          >
            <RepoComponents.Icon />
            Repos
          </CommandItem>
          <CommandItem
            className="flex items-center gap-2 cursor-pointer"
            onSelect={() => nav("/procedures")}
          >
            <ProcedureComponents.Icon />
            Procedures
          </CommandItem>
          {user?.admin && (
            <CommandItem
              className="flex items-center gap-2 cursor-pointer"
              onSelect={() => nav("/users")}
            >
              <UserCircle2 className="w-4 h-4" />
              Users
            </CommandItem>
          )}
        </CommandGroup>

        <CommandSeparator />

        {RESOURCE_TARGETS.map((rt) => (
          <Fragment key={rt}>
            <ResourceGroup type={rt} key={rt} onSelect={nav} />
            <CommandSeparator />
          </Fragment>
        ))}
      </CommandList>
    </CommandDialog>
  );
};

const ResourceGroup = ({
  type,
  onSelect,
}: {
  type: UsableResource;
  onSelect: (path: string) => void;
}) => {
  const data = useRead(`List${type}s`, {}).data;
  const Components = ResourceComponents[type];

  if (!data || !data.length) return;

  return (
    <CommandGroup heading={`${type}s`}>
      {data?.map(({ id }) => {
        return (
          <CommandItem
            key={id}
            className="cursor-pointer flex items-center gap-2"
            onSelect={() => onSelect(`/${usableResourcePath(type)}/${id}`)}
          >
            <Components.Icon id={id} />
            <Components.Name id={id} />
          </CommandItem>
        );
      })}
    </CommandGroup>
  );
};
