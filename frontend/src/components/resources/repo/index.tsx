import { useRead } from "@lib/hooks";
import { RequiredResourceComponents } from "@types";
import { Card, CardHeader } from "@ui/card";
import { FolderGit, GitBranch, Server } from "lucide-react";
import { RepoConfig } from "./config";
import { CloneRepo, PullRepo } from "./actions";
import { DeleteResource, NewResource, ResourceLink } from "../common";
import { RepoTable } from "./table";
import {
  bg_color_class_by_intention,
  repo_state_intention,
  stroke_color_class_by_intention,
} from "@lib/color";
import { cn } from "@lib/utils";
import { HoverCard, HoverCardContent, HoverCardTrigger } from "@ui/hover-card";
import { useServer } from "../server";
import { Types } from "@monitor/client";
import { DashboardPieChart } from "@pages/home/dashboard";

export const useRepo = (id?: string) =>
  useRead("ListRepos", {}, { refetchInterval: 5000 }).data?.find(
    (d) => d.id === id
  );

const RepoIcon = ({ id, size }: { id?: string; size: number }) => {
  const state = useRepo(id)?.info.state;
  const color = stroke_color_class_by_intention(repo_state_intention(state));
  return <GitBranch className={cn(`w-${size} h-${size}`, state && color)} />;
};

export const RepoComponents: RequiredResourceComponents = {
  list_item: (id) => useRepo(id),

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
      const color = bg_color_class_by_intention(repo_state_intention(state));
      return (
        <Card className={cn("w-fit", color)}>
          <CardHeader className="py-0 px-2">{state}</CardHeader>
        </Card>
      );
    },
    Status: ({ id }) => {
      const info = useRepo(id)?.info;
      if (info?.latest_hash && info?.latest_message) {
        return (
          <HoverCard openDelay={200}>
            <HoverCardTrigger asChild>
              <Card className="px-3 py-2 hover:bg-accent/50 transition-colors cursor-pointer">
                <div className="text-muted-foreground text-sm text-nowrap overflow-hidden overflow-ellipsis">
                  latest commit: {info.latest_hash}
                </div>
              </Card>
            </HoverCardTrigger>
            <HoverCardContent align="start">
              <div className="grid">
                <div className="text-muted-foreground">commit message:</div>
                {info.latest_message}
              </div>
            </HoverCardContent>
          </HoverCard>
        );
      } else {
        return <div className="text-muted-foreground">{"Not cloned"}</div>;
      }
    },
  },

  Info: {
    Repo: ({ id }) => {
      const repo = useRepo(id)?.info.repo;
      return (
        <div className="flex items-center gap-2">
          <FolderGit className="w-4 h-4" />
          {repo}
        </div>
      );
    },
    // Branch: ({ id }) => {
    //   const branch = useRepo(id)?.info.branch;
    //   return (
    //     <div className="flex items-center gap-2">
    //       <FolderGit className="w-4 h-4" />
    //       {branch}
    //     </div>
    //   );
    // },
    Server: ({ id }) => {
      const info = useRepo(id)?.info;
      const server = useServer(info?.server_id);
      return server?.id ? (
        <ResourceLink type="Server" id={server?.id} />
      ) : (
        <div className="flex gap-2 items-center">
          <Server className="w-4 h-4" />
          <div>Unknown Server</div>
        </div>
      );
    },
  },

  Actions: { PullRepo, CloneRepo },

  Page: {},

  Config: RepoConfig,

  DangerZone: ({ id }) => <DeleteResource type="Repo" id={id} />,
};
