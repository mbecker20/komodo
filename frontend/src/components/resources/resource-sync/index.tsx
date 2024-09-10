import { useRead } from "@lib/hooks";
import { RequiredResourceComponents } from "@types";
import { Card } from "@ui/card";
import { Clock, FolderSync } from "lucide-react";
import { DeleteResource, NewResource } from "../common";
import { ResourceSyncTable } from "./table";
import { Types } from "@komodo/client";
import { ExecuteSync, RefreshSync } from "./actions";
import { PendingOrConfig } from "./pending-or-config";
import {
  resource_sync_state_intention,
  stroke_color_class_by_intention,
} from "@lib/color";
import { cn } from "@lib/utils";
import { HoverCard, HoverCardContent, HoverCardTrigger } from "@ui/hover-card";
import { fmt_date } from "@lib/formatting";
import { DashboardPieChart } from "@pages/home/dashboard";
import { StatusBadge } from "@components/util";

const useResourceSync = (id?: string) =>
  useRead("ListResourceSyncs", {}, { refetchInterval: 5000 }).data?.find(
    (d) => d.id === id
  );

const ResourceSyncIcon = ({ id, size }: { id?: string; size: number }) => {
  const state = useResourceSync(id)?.info.state;
  const color = stroke_color_class_by_intention(
    resource_sync_state_intention(state)
  );
  return <FolderSync className={cn(`w-${size} h-${size}`, state && color)} />;
};

export const ResourceSyncComponents: RequiredResourceComponents = {
  list_item: (id) => useResourceSync(id),
  use_links: () => undefined,

  Description: () => <>Define resources in git-checked files.</>,

  Dashboard: () => {
    const summary = useRead("GetResourceSyncsSummary", {}).data;
    return (
      <DashboardPieChart
        data={[
          { title: "Ok", intention: "Good", value: summary?.ok ?? 0 },
          {
            title: "Syncing",
            intention: "Warning",
            value: summary?.syncing ?? 0,
          },
          {
            title: "Pending",
            intention: "Neutral",
            value: summary?.pending ?? 0,
          },
          {
            title: "Failed",
            intention: "Critical",
            value: summary?.failed ?? 0,
          },
          {
            title: "Unknown",
            intention: "Unknown",
            value: summary?.unknown ?? 0,
          },
        ]}
      />
    );
  },

  New: () => <NewResource type="ResourceSync" readable_type="Resource Sync" />,

  Table: ({ resources }) => (
    <ResourceSyncTable syncs={resources as Types.ResourceSyncListItem[]} />
  ),

  Icon: ({ id }) => <ResourceSyncIcon id={id} size={4} />,
  BigIcon: ({ id }) => <ResourceSyncIcon id={id} size={8} />,

  Status: {
    State: ({ id }) => {
      const state = useResourceSync(id)?.info.state;
      return (
        <StatusBadge
          text={state}
          intent={resource_sync_state_intention(state)}
        />
      );
    },
    Status: ({ id }) => {
      const info = useResourceSync(id)?.info;
      if (info?.last_sync_hash && info?.last_sync_message) {
        return (
          <HoverCard openDelay={200}>
            <HoverCardTrigger asChild>
              <Card className="px-3 py-2 hover:bg-accent/50 transition-colors cursor-pointer">
                <div className="text-muted-foreground text-sm text-nowrap overflow-hidden overflow-ellipsis">
                  last sync: {info.last_sync_hash}
                </div>
              </Card>
            </HoverCardTrigger>
            <HoverCardContent align="start">
              <div className="grid">
                <div className="text-muted-foreground">commit message:</div>
                {info.last_sync_message}
              </div>
            </HoverCardContent>
          </HoverCard>
        );
      } else {
        return <div className="text-muted-foreground">{"Never synced"}</div>;
      }
    },
  },

  Info: {
    LastSync: ({ id }) => {
      const last_ts = useResourceSync(id)?.info.last_sync_ts;
      return (
        <div className="flex items-center gap-2">
          <Clock className="w-4 h-4" />
          {last_ts ? fmt_date(new Date(last_ts)) : "Never"}
        </div>
      );
    },
  },

  Actions: { RefreshSync, ExecuteSync },

  Page: {},

  Config: PendingOrConfig,

  DangerZone: ({ id }) => <DeleteResource type="ResourceSync" id={id} />,
};
