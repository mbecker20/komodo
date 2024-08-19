import { useExecute, useLocalStorage, useRead } from "@lib/hooks";
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
  AlertTriangle,
} from "lucide-react";
import { Section } from "@components/layouts";
import { RenameServer } from "./actions";
import {
  server_state_intention,
  stroke_color_class_by_intention,
} from "@lib/color";
import { ServerConfig } from "./config";
import { DeploymentTable } from "../deployment/table";
import { ServerTable } from "./table";
import { Link } from "react-router-dom";
import { DeleteResource, NewResource } from "../common";
import { ActionWithDialog, ConfirmButton, StatusBadge } from "@components/util";
import { Button } from "@ui/button";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@ui/tabs";
import { RepoTable } from "../repo/table";
import { DashboardPieChart } from "@pages/home/dashboard";
import { StackTable } from "../stack/table";
import { ResourceComponents } from "..";
import { ServerInfo } from "./info";

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
        state && stroke_color_class_by_intention(server_state_intention(state))
      )}
    />
  );
};

const ConfigOrChildResources = ({ id }: { id: string }) => {
  const [view, setView] = useLocalStorage("server-tabs-v1", "Config");

  const deployments =
    useRead("ListDeployments", {}).data?.filter(
      (deployment) => deployment.info.server_id === id
    ) ?? [];
  const noDeployments = deployments.length === 0;
  const repos =
    useRead("ListRepos", {}).data?.filter(
      (repo) => repo.info.server_id === id
    ) ?? [];
  const noRepos = repos.length === 0;
  const stacks =
    useRead("ListStacks", {}).data?.filter(
      (stack) => stack.info.server_id === id
    ) ?? [];
  const noStacks = stacks.length === 0;

  const noResources = noDeployments && noRepos && noStacks;

  const currentView = view === "Resources" && noResources ? "Config" : view;

  const tabsList = (
    <TabsList className="justify-start w-fit">
      <TabsTrigger value="Config" className="w-[110px]">
        Config
      </TabsTrigger>

      <TabsTrigger value="Info" className="w-[110px]">
        Info
      </TabsTrigger>

      <TabsTrigger
        value="Resources"
        className="w-[110px]"
        disabled={noResources}
      >
        Resources
      </TabsTrigger>
    </TabsList>
  );
  return (
    <Tabs value={currentView} onValueChange={setView} className="grid gap-4">
      <TabsContent value="Config">
        <ServerConfig id={id} titleOther={tabsList} />
      </TabsContent>

      <TabsContent value="Info">
        <ServerInfo id={id} titleOther={tabsList} />
      </TabsContent>

      <TabsContent value="Resources">
        <Section titleOther={tabsList}>
          <Section
            title="Deployments"
            actions={<ResourceComponents.Deployment.New server_id={id} />}
          >
            <DeploymentTable deployments={deployments} />
          </Section>
          <Section
            title="Stacks"
            actions={<ResourceComponents.Stack.New server_id={id} />}
          >
            <StackTable stacks={stacks} />
          </Section>
          <Section
            title="Repos"
            actions={<ResourceComponents.Repo.New server_id={id} />}
          >
            <RepoTable repos={repos} />
          </Section>
        </Section>
      </TabsContent>
    </Tabs>
  );
};

export const ServerComponents: RequiredResourceComponents = {
  list_item: (id) => useServer(id),

  Description: () => (
    <>Connect servers for alerting, building, and deploying.</>
  ),

  Dashboard: () => {
    const summary = useRead("GetServersSummary", {}).data;
    return (
      <DashboardPieChart
        data={[
          { title: "Healthy", intention: "Good", value: summary?.healthy ?? 0 },
          {
            title: "Unhealthy",
            intention: "Warning",
            value: summary?.unhealthy ?? 0,
          },
          {
            title: "Disabled",
            intention: "Neutral",
            value: summary?.disabled ?? 0,
          },
        ]}
      />
    );
  },

  New: () => <NewResource type="Server" />,

  Table: ({ resources }) => (
    <ServerTable servers={resources as Types.ServerListItem[]} />
  ),

  Icon: ({ id }) => <Icon id={id} size={4} />,
  BigIcon: ({ id }) => <Icon id={id} size={8} />,

  Status: {
    State: ({ id }) => {
      const state = useServer(id)?.info.state;
      return (
        <StatusBadge text={state} intent={server_state_intention(state)} />
      );
    },
    Version: ({ id }) => {
      const version = useRead(
        "GetPeripheryVersion",
        { server: id },
        { refetchInterval: 5000 }
      ).data?.version;
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
          {
            enabled: server ? server.info.state !== "Disabled" : false,
            refetchInterval: 5000,
          }
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
        {
          enabled: server ? server.info.state !== "Disabled" : false,
          refetchInterval: 5000,
        }
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
        {
          enabled: server ? server.info.state !== "Disabled" : false,
          refetchInterval: 5000,
        }
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
    Alerts: ({ id }) => {
      return (
        <Link to={`/servers/${id}/alerts`} className="flex gap-2 items-center">
          <AlertTriangle className="w-4 h-4" />
          Alerts
        </Link>
      );
    },
  },

  Actions: {
    Prune: ({ id }) => {
      const { mutate, isPending } = useExecute(`PruneImages`);
      const pruning = useRead(
        "GetServerActionState",
        { server: id },
        { refetchInterval: 5000 }
      ).data?.pruning_images;
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

  Page: {},

  Config: ConfigOrChildResources,

  DangerZone: ({ id }) => (
    <>
      <RenameServer id={id} />
      <DeleteResource type="Server" id={id} />
    </>
  ),
};
