import { useInvalidate, useRead, useWrite } from "@lib/hooks";
import { RequiredResourceComponents } from "@types";
import { Card } from "@ui/card";
import {
  Factory,
  FolderGit,
  GitBranch,
  Loader2,
  RefreshCcw,
  Server,
} from "lucide-react";
import { RepoConfig } from "./config";
import { BuildRepo, CloneRepo, PullRepo } from "./actions";
import { DeleteResource, NewResource, ResourceLink } from "../common";
import { RepoTable } from "./table";
import {
  repo_state_intention,
  stroke_color_class_by_intention,
} from "@lib/color";
import { cn } from "@lib/utils";
import { HoverCard, HoverCardContent, HoverCardTrigger } from "@ui/hover-card";
import { useServer } from "../server";
import { Types } from "@monitor/client";
import { DashboardPieChart } from "@pages/home/dashboard";
import { StatusBadge } from "@components/util";
import { Badge } from "@ui/badge";
import { useToast } from "@ui/use-toast";
import { Button } from "@ui/button";
import { useBuilder } from "../builder";

export const useRepo = (id?: string) =>
  useRead("ListRepos", {}, { refetchInterval: 5000 }).data?.find(
    (d) => d.id === id
  );

export const useFullRepo = (id: string) =>
  useRead("GetRepo", { repo: id }, { refetchInterval: 5000 }).data;

const RepoIcon = ({ id, size }: { id?: string; size: number }) => {
  const state = useRepo(id)?.info.state;
  const color = stroke_color_class_by_intention(repo_state_intention(state));
  return <GitBranch className={cn(`w-${size} h-${size}`, state && color)} />;
};

export const RepoComponents: RequiredResourceComponents = {
  list_item: (id) => useRepo(id),

  Description: () => <>Build using custom scripts. Or anything else.</>,

  Dashboard: () => {
    const summary = useRead("GetReposSummary", {}).data;
    return (
      <DashboardPieChart
        data={[
          { intention: "Good", value: summary?.ok ?? 0, title: "Ok" },
          {
            intention: "Warning",
            value: (summary?.cloning ?? 0) + (summary?.pulling ?? 0),
            title: "Pulling",
          },
          {
            intention: "Critical",
            value: summary?.failed ?? 0,
            title: "Failed",
          },
          {
            intention: "Unknown",
            value: summary?.unknown ?? 0,
            title: "Unknown",
          },
        ]}
      />
    );
  },

  New: ({ server_id }) => <NewResource type="Repo" server_id={server_id} />,

  Table: ({ resources }) => (
    <RepoTable repos={resources as Types.RepoListItem[]} />
  ),

  Icon: ({ id }) => <RepoIcon id={id} size={4} />,
  BigIcon: ({ id }) => <RepoIcon id={id} size={8} />,

  Status: {
    State: ({ id }) => {
      const state = useRepo(id)?.info.state;
      return <StatusBadge text={state} intent={repo_state_intention(state)} />;
    },
    Cloned: ({ id }) => {
      const info = useRepo(id)?.info;
      if (!info?.cloned_hash || info.cloned_hash === info.latest_hash) {
        return null;
      }
      return (
        <HoverCard openDelay={200}>
          <HoverCardTrigger asChild>
            <Card className="px-3 py-2 hover:bg-accent/50 transition-colors cursor-pointer">
              <div className="text-muted-foreground text-sm text-nowrap overflow-hidden overflow-ellipsis">
                cloned: {info.cloned_hash}
              </div>
            </Card>
          </HoverCardTrigger>
          <HoverCardContent align="start">
            <div className="grid">
              <div className="text-muted-foreground">commit message:</div>
              {info.cloned_message}
            </div>
          </HoverCardContent>
        </HoverCard>
      );
    },
    Built: ({ id }) => {
      const info = useRepo(id)?.info;
      const fullInfo = useFullRepo(id)?.info;
      if (!info?.built_hash || info.built_hash === info.latest_hash) {
        return null;
      }
      return (
        <HoverCard openDelay={200}>
          <HoverCardTrigger asChild>
            <Card className="px-3 py-2 hover:bg-accent/50 transition-colors cursor-pointer">
              <div className="text-muted-foreground text-sm text-nowrap overflow-hidden overflow-ellipsis">
                built: {info.built_hash}
              </div>
            </Card>
          </HoverCardTrigger>
          <HoverCardContent align="start">
            <div className="grid gap-2">
              <Badge
                variant="secondary"
                className="w-fit text-muted-foreground"
              >
                commit message
              </Badge>
              {fullInfo?.built_message}
            </div>
          </HoverCardContent>
        </HoverCard>
      );
    },
    Latest: ({ id }) => {
      const info = useRepo(id)?.info;
      const fullInfo = useFullRepo(id)?.info;
      if (!info?.latest_hash) {
        return null;
      }
      return (
        <HoverCard openDelay={200}>
          <HoverCardTrigger asChild>
            <Card className="px-3 py-2 hover:bg-accent/50 transition-colors cursor-pointer">
              <div className="text-muted-foreground text-sm text-nowrap overflow-hidden overflow-ellipsis">
                latest: {info.latest_hash}
              </div>
            </Card>
          </HoverCardTrigger>
          <HoverCardContent align="start">
            <div className="grid gap-2">
              <Badge
                variant="secondary"
                className="w-fit text-muted-foreground"
              >
                commit message
              </Badge>
              {fullInfo?.latest_message}
            </div>
          </HoverCardContent>
        </HoverCard>
      );
    },
    Refresh: ({ id }) => {
      const { toast } = useToast();
      const inv = useInvalidate();
      const { mutate, isPending } = useWrite("RefreshRepoCache", {
        onSuccess: () => {
          inv(["ListRepos"], ["GetRepo", { repo: id }]);
          toast({ title: "Refreshed repo status cache" });
        },
      });
      return (
        <Button
          variant="outline"
          size="icon"
          onClick={() => {
            mutate({ repo: id });
            toast({ title: "Triggered refresh of repo status cache" });
          }}
        >
          {isPending ? (
            <Loader2 className="w-4 h-4 animate-spin" />
          ) : (
            <RefreshCcw className="w-4 h-4" />
          )}
        </Button>
      );
    },
  },

  Info: {
    Server: ({ id }) => {
      const info = useRepo(id)?.info;
      const server = useServer(info?.server_id);
      return server?.id ? (
        <ResourceLink type="Server" id={server?.id} />
      ) : (
        <div className="flex gap-2 items-center">
          <Server className="w-4 h-4" />
          <div>No Server</div>
        </div>
      );
    },
    Builder: ({ id }) => {
      const info = useRepo(id)?.info;
      const builder = useBuilder(info?.builder_id);
      return builder?.id ? (
        <ResourceLink type="Builder" id={builder?.id} />
      ) : (
        <div className="flex gap-2 items-center">
          <Factory className="w-4 h-4" />
          <div>No Builder</div>
        </div>
      );
    },
    Repo: ({ id }) => {
      const repo = useRepo(id)?.info.repo;
      return (
        <div className="flex items-center gap-2">
          <FolderGit className="w-4 h-4" />
          {repo}
        </div>
      );
    },
    Branch: ({ id }) => {
      const branch = useRepo(id)?.info.branch;
      return (
        <div className="flex items-center gap-2">
          <GitBranch className="w-4 h-4" />
          {branch}
        </div>
      );
    },
  },

  Actions: { BuildRepo, PullRepo, CloneRepo },

  Page: {},

  Config: RepoConfig,

  DangerZone: ({ id }) => <DeleteResource type="Repo" id={id} />,
};
