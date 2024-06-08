import { useRead } from "@lib/hooks";
import { RequiredResourceComponents } from "@types";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@ui/card";
import { FolderSync } from "lucide-react";
import { Link } from "react-router-dom";
import { DeleteResource, NewResource } from "../common";
import { ResourceSyncTable } from "./table";
import { Types } from "@monitor/client";
import { ResourceSyncConfig } from "./config";
import { useState } from "react";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@ui/tabs";
import { ExecuteSync, RefreshSync } from "./actions";
import { sanitizeOnlySpan, sync_no_changes } from "@lib/utils";
import { Section } from "@components/layouts";

const useResourceSync = (id?: string) =>
  useRead("ListResourceSyncs", {}).data?.find((d) => d.id === id);

const PENDING_TYPE_KEYS: Array<[string, string]> = [
  ["Server", "server_updates"],
  ["Deployment", "deployment_updates"],
  ["Build", "build_updates"],
  ["Repo", "repo_updates"],
  ["Procedure", "procedure_updates"],
  ["Alerter", "alerter_updates"],
  ["Builder", "builder_updates"],
  ["Server Template", "server_template_updates"],
  ["Resource Sync", "resource_sync_updates"],
  ["Variable", "variable_updates"],
  ["User Group", "user_group_updates"],
];

const PendingOrConfig = ({ id }: { id: string }) => {
  const [view, setView] = useState("Pending");

  const sync = useRead("GetResourceSync", { sync: id }).data;

  const pendingDisabled = !sync || sync_no_changes(sync);
  const currentView = view === "Pending" && pendingDisabled ? "Config" : view;

  const tabsList = (
    <TabsList className="justify-start w-fit">
      <TabsTrigger
        value="Pending"
        className="w-[110px]"
        disabled={pendingDisabled}
      >
        Pending
      </TabsTrigger>
      <TabsTrigger value="Config" className="w-[110px]">
        Config
      </TabsTrigger>
    </TabsList>
  );
  return (
    <Tabs value={currentView} onValueChange={setView} className="grid gap-4">
      <TabsContent value="Config">
        <ResourceSyncConfig id={id} titleOther={tabsList} />
      </TabsContent>
      <TabsContent value="Pending">
        <Section titleOther={tabsList}>
          {PENDING_TYPE_KEYS.map(([type, key]) => (
            <PendingView
              key={type}
              type={type}
              pending={sync?.info?.pending?.[key]}
            />
          ))}
        </Section>
      </TabsContent>
    </Tabs>
  );
};

const PendingView = ({
  type,
  pending,
}: {
  type: string;
  pending: Types.SyncUpdate | undefined;
}) => {
  if (!pending) return;

  return (
    <Card>
      <CardHeader className="flex items-center justify-between gap-4">
        <CardTitle>{type} Updates</CardTitle>
        <div className="flex gap-4 items-center">
          {pending.to_create && (
            <div className="flex gap-2 items-center">
              To Create: {pending.to_create}
            </div>
          )}
          {pending.to_update && (
            <div className="flex gap-2 items-center">
              To Update: {pending.to_update}
            </div>
          )}
          {pending.to_delete && (
            <div className="flex gap-2 items-center">
              To Delete: {pending.to_delete}
            </div>
          )}
        </div>
      </CardHeader>
      <CardContent>
        <pre
          dangerouslySetInnerHTML={{
            __html: sanitizeOnlySpan(pending.log),
          }}
        />
      </CardContent>
    </Card>
  );
};

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

  Status: {},

  Info: {},

  Actions: { RefreshSync, ExecuteSync },

  Page: {},

  Config: PendingOrConfig,

  DangerZone: ({ id }) => <DeleteResource type="ResourceSync" id={id} />,
};
