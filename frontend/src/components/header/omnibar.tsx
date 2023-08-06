import { useRead } from "@hooks";
import { AlerterName } from "@resources/alerter";
import { BuildName } from "@resources/build/util";
import { BuilderName } from "@resources/builder";
import {
  DeploymentName,
  DeploymentStatusIcon,
} from "@resources/deployment/util";
import { RepoName } from "@resources/repo";
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
import { AlarmClock, Factory, GitBranch, Hammer, Search } from "lucide-react";
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
  const builders = useRead("ListBuilders", {}).data;
  const alerters = useRead("ListAlerters", {}).data;
  const repos = useRead("ListRepos", {}).data;

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
                  <Hammer className="w-4 h-4" />
                  <BuildName id={id} />
                </CommandItem>
              );
            })}
          </CommandGroup>
          <CommandGroup heading="Builders">
            {builders?.map(({ id }) => {
              return (
                <CommandItem
                  key={id}
                  className="flex items-center gap-2"
                  onSelect={nav(`/builders/${id}`)}
                >
                  <Factory className="w-4 h-4" />
                  <BuilderName id={id} />
                </CommandItem>
              );
            })}
          </CommandGroup>
          <CommandGroup heading="Alerters">
            {alerters?.map(({ id }) => {
              return (
                <CommandItem
                  key={id}
                  className="flex items-center gap-2"
                  onSelect={nav(`/alerters/${id}`)}
                >
                  <AlarmClock className="w-4 h-4" />
                  <AlerterName id={id} />
                </CommandItem>
              );
            })}
          </CommandGroup>
          <CommandGroup heading="Repos">
            {repos?.map(({ id }) => {
              return (
                <CommandItem
                  key={id}
                  className="flex items-center gap-2"
                  onSelect={nav(`/repos/${id}`)}
                >
                  <GitBranch className="w-4 h-4" />
                  <RepoName id={id} />
                </CommandItem>
              );
            })}
          </CommandGroup>
        </CommandList>
      </CommandDialog>
    </>
  );
};
