import { useRead } from "@lib/hooks";
import { RequiredResourceComponents } from "@types";
import { Card, CardDescription, CardHeader, CardTitle } from "@ui/card";
import { Clock, FolderSync } from "lucide-react";
import { Link } from "react-router-dom";
import { DeleteResource, NewResource } from "../common";
import { ResourceSyncTable } from "./table";
import { Types } from "@monitor/client";
import { ExecuteSync, RefreshSync } from "./actions";
import { PendingOrConfig } from "./pending-or-config";
import {
  bg_color_class_by_intention,
  resource_sync_state_intention,
} from "@lib/color";
import { cn } from "@lib/utils";
import { HoverCard, HoverCardContent, HoverCardTrigger } from "@ui/hover-card";
import { fmt_date } from "@lib/formatting";

const useResourceSync = (id?: string) =>
  useRead("ListResourceSyncs", {}, { refetchInterval: 5000 }).data?.find(
    (d) => d.id === id
  );

export const ResourceSyncComponents: RequiredResourceComponents = {
  list_item: (id) => useResourceSync(id),

  Dashboard: () => {
    const syncs_count = useRead("ListResourceSyncs", {}).data?.length;
    return (
      <Link to="/resource-syncs/" className="w-full">
        <Card className="hover:bg-accent/50 transition-colors cursor-pointer">
          <CardHeader>
            <div className="flex justify-between">
              <div>
                <CardTitle>Resource Syncs</CardTitle>
                <CardDescription>{syncs_count} Total</CardDescription>
              </div>
              <FolderSync className="w-4 h-4" />
            </div>
          </CardHeader>
        </Card>
      </Link>
    );
  },

  New: () => <NewResource type="ResourceSync" readable_type="Resource Sync" />,

  Table: ({ resources }) => (
    <ResourceSyncTable syncs={resources as Types.ResourceSyncListItem[]} />
  ),

  Icon: () => <FolderSync className="w-4 h-4" />,
  BigIcon: () => <FolderSync className="w-8 h-8" />,

  Status: {
    State: ({ id }) => {
      const state = useResourceSync(id)?.info.state;
      const color = bg_color_class_by_intention(
        resource_sync_state_intention(state)
      );
      return (
        <Card className={cn("w-fit", color)}>
          <CardHeader className="py-0 px-2">{state}</CardHeader>
        </Card>
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
