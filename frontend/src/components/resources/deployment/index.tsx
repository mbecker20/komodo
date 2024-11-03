import { useLocalStorage, useRead } from "@lib/hooks";
import { Types } from "komodo_client";
import { RequiredResourceComponents } from "@types";
import { HardDrive, Rocket, Server } from "lucide-react";
import { cn } from "@lib/utils";
import { useServer } from "../server";
import {
  DeployDeployment,
  StartStopDeployment,
  DestroyDeployment,
  RestartDeployment,
  PauseUnpauseDeployment,
  PullDeployment,
} from "./actions";
import { DeploymentLogs } from "./log";
import {
  deployment_state_intention,
  stroke_color_class_by_intention,
} from "@lib/color";
import { DeploymentTable } from "./table";
import { DeleteResource, NewResource, ResourceLink } from "../common";
import { RunBuild } from "../build/actions";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@ui/tabs";
import { DeploymentConfig } from "./config";
import { DashboardPieChart } from "@pages/home/dashboard";
import { ResourcePageHeader, StatusBadge } from "@components/util";
import { RenameResource } from "@components/config/util";

// const configOrLog = atomWithStorage("config-or-log-v1", "Config");

export const useDeployment = (id?: string) =>
  useRead("ListDeployments", {}, { refetchInterval: 10_000 }).data?.find(
    (d) => d.id === id
  );

export const useFullDeployment = (id: string) =>
  useRead("GetDeployment", { deployment: id }, { refetchInterval: 10_000 })
    .data;

const ConfigOrLog = ({ id }: { id: string }) => {
  // const [view, setView] = useAtom(configOrLog);
  const [view, setView] = useLocalStorage("deployment-tabs-v1", "Config");
  const state = useDeployment(id)?.info.state;
  const logsDisabled =
    state === undefined ||
    state === Types.DeploymentState.Unknown ||
    state === Types.DeploymentState.NotDeployed;
  return (
    <Tabs
      value={logsDisabled ? "Config" : view}
      onValueChange={setView}
      className="grid gap-4"
    >
      <TabsContent value="Config">
        <DeploymentConfig
          id={id}
          titleOther={
            <TabsList className="justify-start w-fit">
              <TabsTrigger value="Config" className="w-[110px]">
                Config
              </TabsTrigger>
              <TabsTrigger
                value="Log"
                className="w-[110px]"
                disabled={logsDisabled}
              >
                Log
              </TabsTrigger>
            </TabsList>
          }
        />
      </TabsContent>
      <TabsContent value="Log">
        <DeploymentLogs
          id={id}
          titleOther={
            <TabsList className="justify-start w-fit">
              <TabsTrigger value="Config" className="w-[110px]">
                Config
              </TabsTrigger>
              <TabsTrigger
                value="Log"
                className="w-[110px]"
                disabled={logsDisabled}
              >
                Log
              </TabsTrigger>
            </TabsList>
          }
        />
      </TabsContent>
    </Tabs>
  );
};

const DeploymentIcon = ({ id, size }: { id?: string; size: number }) => {
  const state = useDeployment(id)?.info.state;
  const color = stroke_color_class_by_intention(
    deployment_state_intention(state)
  );
  return <Rocket className={cn(`w-${size} h-${size}`, state && color)} />;
};

export const DeploymentComponents: RequiredResourceComponents = {
  list_item: (id) => useDeployment(id),
  resource_links: (resource) =>
    (resource.config as Types.DeploymentConfig).links,

  Description: () => <>Deploy containers on your servers.</>,

  Dashboard: () => {
    const summary = useRead("GetDeploymentsSummary", {}).data;
    const all = [
      summary?.running ?? 0,
      summary?.stopped ?? 0,
      summary?.unhealthy ?? 0,
      summary?.unknown ?? 0,
    ];
    const [running, stopped, unhealthy, unknown] = all;
    return (
      <DashboardPieChart
        data={[
          all.every((item) => item === 0) && {
            title: "Not Deployed",
            intention: "Neutral",
            value: summary?.not_deployed ?? 0,
          },
          { intention: "Good", value: running, title: "Running" },
          {
            title: "Stopped",
            intention: "Warning",
            value: stopped,
          },
          {
            title: "Unhealthy",
            intention: "Critical",
            value: unhealthy,
          },
          {
            title: "Unknown",
            intention: "Unknown",
            value: unknown,
          },
        ]}
      />
    );
  },

  New: ({ server_id: _server_id, build_id }) => {
    const servers = useRead("ListServers", {}).data;
    const server_id = _server_id
      ? _server_id
      : servers && servers.length === 1
        ? servers[0].id
        : undefined;
    return (
      <NewResource
        type="Deployment"
        server_id={server_id}
        build_id={build_id}
      />
    );
  },

  Table: ({ resources }) => {
    return (
      <DeploymentTable deployments={resources as Types.DeploymentListItem[]} />
    );
  },

  Icon: ({ id }) => <DeploymentIcon id={id} size={4} />,
  BigIcon: ({ id }) => <DeploymentIcon id={id} size={8} />,

  State: ({ id }) => {
    const state =
      useDeployment(id)?.info.state ?? Types.DeploymentState.Unknown;
    return (
      <StatusBadge text={state} intent={deployment_state_intention(state)} />
    );
  },

  Status: {},

  Info: {
    Image: ({ id }) => {
      const info = useDeployment(id)?.info;
      return info?.build_id ? (
        <ResourceLink type="Build" id={info.build_id} />
      ) : (
        <div className="flex gap-2 items-center text-sm">
          <HardDrive className="w-4 h-4" />
          <div>{info?.image || "N/A"}</div>
        </div>
      );
    },
    Server: ({ id }) => {
      const info = useDeployment(id)?.info;
      const server = useServer(info?.server_id);
      return server?.id ? (
        <ResourceLink type="Server" id={server?.id} />
      ) : (
        <div className="flex gap-2 items-center text-sm">
          <Server className="w-4 h-4" />
          <div>Unknown Server</div>
        </div>
      );
    },
  },

  Actions: {
    RunBuild: ({ id }) => {
      const build_id = useDeployment(id)?.info.build_id;
      if (!build_id) return null;
      return <RunBuild id={build_id} />;
    },
    DeployDeployment,
    PullDeployment,
    RestartDeployment,
    PauseUnpauseDeployment,
    StartStopDeployment,
    DestroyDeployment,
  },

  Page: {},

  Config: ConfigOrLog,

  DangerZone: ({ id }) => (
    <>
      <RenameResource type="Deployment" id={id} />
      <DeleteResource type="Deployment" id={id} />
    </>
  ),

  ResourcePageHeader: ({ id }) => {
    const deployment = useDeployment(id);

    return (
      <ResourcePageHeader
        intent={deployment_state_intention(deployment?.info.state)}
        icon={<DeploymentIcon id={id} size={8} />}
        name={deployment?.name}
        state={
          deployment?.info.state === Types.DeploymentState.NotDeployed
            ? "Not Deployed"
            : deployment?.info.state
        }
        status={deployment?.info.status}
      />
    );
  },
};
