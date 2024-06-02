import { useExecute, useRead } from "@lib/hooks";
import { cn } from "@lib/utils";
import { Types } from "@monitor/client";
import { RequiredResourceComponents } from "@types";
import {
  Server,
  Cpu,
  MemoryStick,
  Database,
  Scissors,
  XOctagon,
  AreaChart,
  Milestone,
} from "lucide-react";
import { Section } from "@components/layouts";
import { RenameServer } from "./actions";
import {
  bg_color_class_by_intention,
  fill_color_class_by_intention,
  server_state_intention,
} from "@lib/color";
import { ServerConfig } from "./config";
import { DeploymentTable } from "../deployment/table";
import { ServerTable } from "./table";
import { ServersChart } from "./dashboard";
import { Link } from "react-router-dom";
import { DeleteResource, NewResource } from "../common";
import { ActionWithDialog, ConfirmButton } from "@components/util";
import { Card, CardHeader } from "@ui/card";
import { Button } from "@ui/button";
import { useState } from "react";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@ui/tabs";
import { RepoTable } from "../repo/table";
import { ResourceComponents } from "..";

export const useServer = (id?: string) =>
  useRead("ListServers", {}, { refetchInterval: 5000 }).data?.find(
    (d) => d.id === id
  );

const Icon = ({ id, size }: { id?: string; size: number }) => {
  const state = useServer(id)?.info.state;
  return (
    <Server
      className={cn(
        `w-${size} h-${size}`,
        id && fill_color_class_by_intention(server_state_intention(state))
      )}
    />
  );
};

const ConfigOrChildResources = ({ id }: { id: string }) => {
  const [view, setView] = useState("Config");
  const deployments = useRead("ListDeployments", {}).data?.filter(
    (deployment) => deployment.info.server_id === id
  );
  const deploymentsDisabled = (deployments?.length || 0) === 0;
  const repos = useRead("ListRepos", {}).data?.filter(
    (repo) => repo.info.server_id === id
  );
  const reposDisabled = (repos?.length || 0) === 0;
  const currentView =
    (view === "Deployments" && deploymentsDisabled) ||
    (view === "Repos" && reposDisabled)
      ? "Config"
      : view;
  const tabsList = (
    <TabsList className="justify-start w-fit">
      <TabsTrigger value="Config" className="w-[110px]">
        Config
      </TabsTrigger>
      <TabsTrigger
        value="Deployments"
        className="w-[110px]"
        disabled={deploymentsDisabled}
      >
        Deployments
      </TabsTrigger>
      <TabsTrigger value="Repos" className="w-[110px]" disabled={reposDisabled}>
        Repos
      </TabsTrigger>
    </TabsList>
  );
  return (
    <Tabs value={currentView} onValueChange={setView} className="grid gap-4">
      <TabsContent value="Config">
        <ServerConfig id={id} titleOther={tabsList} />
      </TabsContent>

      <TabsContent value="Deployments">
        <Section
          titleOther={tabsList}
          actions={<ResourceComponents.Deployment.New server_id={id} />}
        >
          <DeploymentTable deployments={deployments} />
        </Section>
      </TabsContent>

      <TabsContent value="Repos">
        <Section
          titleOther={tabsList}
          actions={<ResourceComponents.Repo.New server_id={id} />}
        >
          <RepoTable repos={repos} />
        </Section>
      </TabsContent>
    </Tabs>
  );
};

export const ServerComponents: RequiredResourceComponents = {
  list_item: (id) => useServer(id),

  Dashboard: ServersChart,

  New: () => <NewResource type="Server" />,

  Table: ServerTable,

  Icon: ({ id }) => <Icon id={id} size={4} />,
  BigIcon: ({ id }) => <Icon id={id} size={8} />,

  Status: {
    State: ({ id }) => {
      const state = useServer(id)?.info.state;
      const color = bg_color_class_by_intention(server_state_intention(state));
      return (
        <Card className={cn("w-fit", color)}>
          <CardHeader className="py-0 px-2">
            {state === Types.ServerState.NotOk ? "Not Ok" : state}
          </CardHeader>
        </Card>
      );
    },
    Version: ({ id }) => {
      const version = useRead("GetPeripheryVersion", { server: id }).data
        ?.version;
      const _version =
        version === undefined || version === "unknown" ? "unknown" : version;
      return (
        <div className="flex items-center gap-2">
          <Milestone className="w-4 h-4" />
          {_version}
        </div>
      );
    },
    Stats: ({ id }) => (
      <Link to={`/servers/${id}/stats`}>
        <Button variant="link" className="flex gap-2 items-center p-0">
          <AreaChart className="w-4 h-4" />
          Stats
        </Button>
      </Link>
    ),
  },

  Info: {
    Cpu: ({ id }) => {
      const server = useServer(id);
      const core_count =
        useRead(
          "GetSystemInformation",
          { server: id },
          { enabled: server ? server.info.state !== "Disabled" : false }
        ).data?.core_count ?? 0;
      return (
        <Link to={`/servers/${id}/stats`} className="flex gap-2 items-center">
          <Cpu className="w-4 h-4" />
          {core_count || "N/A"} Core{core_count > 1 ? "s" : ""}
        </Link>
      );
    },
    Mem: ({ id }) => {
      const server = useServer(id);
      const stats = useRead(
        "GetSystemStats",
        { server: id },
        { enabled: server ? server.info.state !== "Disabled" : false }
      ).data;
      return (
        <Link to={`/servers/${id}/stats`} className="flex gap-2 items-center">
          <MemoryStick className="w-4 h-4" />
          {stats?.mem_total_gb.toFixed(2) ?? "N/A"} GB
        </Link>
      );
    },
    Disk: ({ id }) => {
      const server = useServer(id);
      const stats = useRead(
        "GetSystemStats",
        { server: id },
        { enabled: server ? server.info.state !== "Disabled" : false }
      ).data;
      const disk_total_gb = stats?.disks.reduce(
        (acc, curr) => acc + curr.total_gb,
        0
      );
      return (
        <Link to={`/servers/${id}/stats`} className="flex gap-2 items-center">
          <Database className="w-4 h-4" />
          {disk_total_gb?.toFixed(2) ?? "N/A"} GB
        </Link>
      );
    },
  },

  Actions: {
    Prune: ({ id }) => {
      const { mutate, isPending } = useExecute(`PruneImages`);
      const pruning = useRead("GetServerActionState", { server: id }).data
        ?.pruning_images;
      const pending = isPending || pruning;
      return (
        <ConfirmButton
          title="Prune Images"
          icon={<Scissors className="w-4 h-4" />}
          onClick={() => mutate({ server: id })}
          loading={pending}
          disabled={pending}
        />
      );
    },
    StopAll: ({ id }) => {
      const server = useServer(id);
      const { mutate, isPending } = useExecute(`StopAllContainers`);
      const stopping = useRead(
        "GetServerActionState",
        { server: id },
        { refetchInterval: 5000 }
      ).data?.stopping_containers;
      const pending = isPending || stopping;
      return (
        server && (
          <ActionWithDialog
            name={server?.name}
            title="Stop Containers"
            icon={<XOctagon className="w-4 h-4" />}
            onClick={() => mutate({ server: id })}
            disabled={pending}
            loading={pending}
          />
        )
      );
    },
  },

  Page: {
    // Alerts: ({ id }) => {
    //   const alerts = useRead("ListAlerts", {
    //     query: { "target.type": "Server", "target.id": id },
    //   }).data?.alerts.slice(0, 3);
    //   return (
    //     (alerts?.length || 0) > 0 && (
    //       <Section
    //         title="Alerts"
    //         icon={<AlertTriangle className="w-4 h-4" />}
    //         actions={
    //           <Link to={`/servers/${id}/alerts`}>
    //             <Button variant="secondary" size="icon">
    //               <ExternalLink className="w-4 h-4" />
    //             </Button>
    //           </Link>
    //         }
    //       >
    //         <AlertsTable alerts={alerts ?? []} />
    //       </Section>
    //     )
    //   );
    // },
  },

  Config: ConfigOrChildResources,

  DangerZone: ({ id }) => (
    <>
      <RenameServer id={id} />
      <DeleteResource type="Server" id={id} />
    </>
  ),
};
