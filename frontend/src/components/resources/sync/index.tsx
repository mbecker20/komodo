import { useRead } from "@lib/hooks";
import { RequiredResourceComponents } from "@types";
import { Card, CardDescription, CardHeader, CardTitle } from "@ui/card";
import { FolderSync } from "lucide-react";
import { Link } from "react-router-dom";
import { DeleteResource, NewResource } from "../common";
import { ResourceSyncTable } from "./table";
import { Types } from "@monitor/client";
import { ResourceSyncConfig } from "./config";
import { useState } from "react";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@ui/tabs";

const useResourceSync = (id?: string) =>
  useRead("ListResourceSyncs", {}).data?.find((d) => d.id === id);

const PendingOrConfig = ({ id }: { id: string }) => {
  const [view, setView] = useState("Pending");

  const pendingDisabled = true;
  const currentView = view === "Pending" && pendingDisabled ? "Config" : view;

  const tabsList = (
    <TabsList className="justify-start w-fit">
      <TabsTrigger value="Pending" className="w-[110px]" disabled={true}>
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
    </Tabs>
  );
};

export const ResourceSyncComponents: RequiredResourceComponents = {
  list_item: (id) => useResourceSync(id),

  Dashboard: () => {
    const syncs_count = useRead("ListResourceSyncs", {}).data?.length;
    return (
      <Link to="/syncs/" className="w-full">
        <Card className="hover:bg-accent/50 transition-colors cursor-pointer">
          <CardHeader>
            <div className="flex justify-between">
              <div>
                <CardTitle>Syncs</CardTitle>
                <CardDescription>{syncs_count} Total</CardDescription>
              </div>
              <FolderSync className="w-4 h-4" />
            </div>
          </CardHeader>
        </Card>
      </Link>
    );
  },

  New: () => <NewResource type="ResourceSync" />,

  Table: ({ resources }) => (
    <ResourceSyncTable syncs={resources as Types.ResourceSyncListItem[]} />
  ),

  Icon: () => <FolderSync className="w-4 h-4" />,
  BigIcon: () => <FolderSync className="w-8 h-8" />,

  Status: {},

  Info: {},

  Actions: {},

  Page: {},

  Config: PendingOrConfig,

  DangerZone: ({ id }) => <DeleteResource type="ResourceSync" id={id} />,
};
