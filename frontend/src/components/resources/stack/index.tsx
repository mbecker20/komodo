import { useRead } from "@lib/hooks";
import { RequiredResourceComponents } from "@types";
import { Card, CardHeader } from "@ui/card";
import { FolderGit, GitBranch, Server } from "lucide-react";
import { StackConfig } from "./config";
import { DeleteResource, NewResource, ResourceLink } from "../common";
import { StackTable } from "./table";
import {
  bg_color_class_by_intention,
  stack_state_intention,
  stroke_color_class_by_intention,
} from "@lib/color";
import { cn } from "@lib/utils";
import { HoverCard, HoverCardContent, HoverCardTrigger } from "@ui/hover-card";
import { StackDashboard } from "./dashboard";
import { useServer } from "../server";
import { Types } from "@monitor/client";
import { DeployStack, DestroyStack } from "./actions";

export const useStack = (id?: string) =>
  useRead("ListStacks", {}, { refetchInterval: 5000 }).data?.find(
    (d) => d.id === id
  );

const StackIcon = ({ id, size }: { id?: string; size: number }) => {
  const state = useStack(id)?.info.state;
  const color = stroke_color_class_by_intention(stack_state_intention(state));
  return <GitBranch className={cn(`w-${size} h-${size}`, state && color)} />;
};

export const StackComponents: RequiredResourceComponents = {
  list_item: (id) => useStack(id),

  Dashboard: StackDashboard,

  New: ({ server_id }) => <NewResource type="Stack" server_id={server_id} />,

  Table: ({ resources }) => (
    <StackTable stacks={resources as Types.StackListItem[]} />
  ),

  Icon: ({ id }) => <StackIcon id={id} size={4} />,
  BigIcon: ({ id }) => <StackIcon id={id} size={8} />,

  Status: {
    State: ({ id }) => {
      const state = useStack(id)?.info.state;
      const color = bg_color_class_by_intention(stack_state_intention(state));
      return (
        <Card className={cn("w-fit", color)}>
          <CardHeader className="py-0 px-2">{state}</CardHeader>
        </Card>
      );
    },
    Status: ({ id }) => {
      const info = useStack(id)?.info;
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
    Stack: ({ id }) => {
      const repo = useStack(id)?.info.repo;
      return (
        <div className="flex items-center gap-2">
          <FolderGit className="w-4 h-4" />
          {repo}
        </div>
      );
    },
    // Branch: ({ id }) => {
    //   const branch = useStack(id)?.info.branch;
    //   return (
    //     <div className="flex items-center gap-2">
    //       <FolderGit className="w-4 h-4" />
    //       {branch}
    //     </div>
    //   );
    // },
    Server: ({ id }) => {
      const info = useStack(id)?.info;
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

  Actions: {},

  Page: {
    DeployStack,
    DestroyStack,
  },

  Config: StackConfig,

  DangerZone: ({ id }) => <DeleteResource type="Stack" id={id} />,
};
