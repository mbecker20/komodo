import { useRead } from "@hooks";
import { BuildName } from "@resources/build/util";
import {
  DeploymentName,
  DeploymentStatusIcon,
} from "@resources/deployment/util";
import { ServerName, ServerStatusIcon } from "@resources/server/util";
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
import { Search } from "lucide-react";
import { useState, useEffect } from "react";
import { useNavigate } from "react-router-dom";

export const Omnibar = () => {
  const [open, set] = useState(false);
  const navigate = useNavigate();
  const nav = (path: string) => () => {
    navigate(path);
    set(false);
  };

  useEffect(() => {
    const down = (e: KeyboardEvent) => {
      if (e.key === "s" && e.shiftKey) set(true);
    };
    document.addEventListener("keydown", down);
    return () => document.removeEventListener("keydown", down);
  }, []);

  const deployments = useRead("ListDeployments", {}).data;
  const builds = useRead("ListBuilds", {}).data;
  const servers = useRead("ListServers", {}).data;

  return (
    <>
      <Button variant="ghost" onClick={() => set(true)}>
        <Search className="w-4 h-4" />
      </Button>
      <CommandDialog open={open} onOpenChange={set}>
        <CommandInput placeholder="Type a command or search..." />
        <CommandList>
          <CommandEmpty>No results found.</CommandEmpty>
          <CommandGroup heading="Deployments">
            {deployments?.map(({ id }) => {
              return (
                <CommandItem
                  key={id}
                  className="flex items-center gap-2"
                  onSelect={nav(`/deployments/${id}`)}
                >
                  <DeploymentStatusIcon deploymentId={id} />
                  <DeploymentName deploymentId={id} />
                </CommandItem>
              );
            })}
          </CommandGroup>
          <CommandSeparator />
          <CommandGroup heading="Servers">
            {servers?.map(({ id }) => {
              return (
                <CommandItem
                  key={id}
                  className="flex items-center gap-2"
                  onSelect={nav(`/servers/${id}`)}
                >
                  <ServerStatusIcon serverId={id} />
                  <ServerName serverId={id} />
                </CommandItem>
              );
            })}
          </CommandGroup>
          <CommandSeparator />
          <CommandGroup heading="Builds">
            {builds?.map(({ id }) => {
              return (
                <CommandItem
                  key={id}
                  className="flex items-center gap-2"
                  onSelect={nav(`/builds/${id}`)}
                >
                  <BuildName id={id} />
                </CommandItem>
              );
            })}
          </CommandGroup>
        </CommandList>
      </CommandDialog>
    </>
  );
};
