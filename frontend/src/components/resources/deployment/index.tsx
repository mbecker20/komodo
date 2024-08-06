import { useRead } from "@lib/hooks";
import { Types } from "@monitor/client";
import { RequiredResourceComponents } from "@types";
import { AlertTriangle, HardDrive, Rocket, Server } from "lucide-react";
import { cn } from "@lib/utils";
import { useServer } from "../server";
import {
  DeployContainer,
  StartOrStopContainer,
  RemoveContainer,
  DeleteDeployment,
  RenameDeployment,
} from "./actions";
import { DeploymentLogs } from "./log";
import {
  deployment_state_intention,
  stroke_color_class_by_intention,
} from "@lib/color";
import { DeploymentTable } from "./table";
import { NewResource, ResourceLink } from "../common";
import { RunBuild } from "../build/actions";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@ui/tabs";
import { DeploymentConfig } from "./config";
import { useState } from "react";
import { Link } from "react-router-dom";
import { DashboardPieChart } from "@pages/home/dashboard";
import { StatusBadge } from "@components/util";

// const configOrLog = atomWithStorage("config-or-log-v1", "Config");

export const useDeployment = (id?: string) =>
  useRead("ListDeployments", {}, { refetchInterval: 5000 }).data?.find(
    (d) => d.id === id
  );

const ConfigOrLog = ({ id }: { id: string }) => {
  // const [view, setView] = useAtom(configOrLog);
  const [view, setView] = useState("Config");
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

  Dashboard: () => {
    const summary = useRead("GetDeploymentsSummary", {}).data;
    return (
      <DashboardPieChart
        data={[
          { intention: "Good", value: summary?.running ?? 0, title: "Running" },
          {
            intention: "Critical",
            value: summary?.stopped ?? 0,
            title: "Stopped",
          },
          {
            intention: "Neutral",
            value: summary?.not_deployed ?? 0,
            title: "Not Deployed",
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

  New: ({ server_id, build_id }) => (
    <NewResource type="Deployment" server_id={server_id} build_id={build_id} />
  ),

  Table: ({ resources }) => {
    return (
      <DeploymentTable deployments={resources as Types.DeploymentListItem[]} />
    );
  },

  Icon: ({ id }) => <DeploymentIcon id={id} size={4} />,
  BigIcon: ({ id }) => <DeploymentIcon id={id} size={8} />,

  Status: {
    State: ({ id }) => {
      const state =
        useDeployment(id)?.info.state ?? Types.DeploymentState.Unknown;

      const intention = deployment_state_intention(state);
      return <StatusBadge text={state} intent={intention} />;
    },
    Status: ({ id }) => {
      const status = useDeployment(id)?.info.status;
      return (
        status && <p className="text-sm text-muted-foreground">{status}</p>
      );
    },
  },

  Info: {
    Image: ({ id }) => {
      const info = useDeployment(id)?.info;
      return info?.build_id ? (
        <ResourceLink type="Build" id={info.build_id} />
      ) : (
        <div className="flex gap-2 items-center">
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
        <div className="flex gap-2 items-center">
          <Server className="w-4 h-4" />
          <div>Unknown Server</div>
        </div>
      );
    },
    Alerts: ({ id }) => {
      return (
        <Link
          to={`/deployments/${id}/alerts`}
          className="flex gap-2 items-center"
        >
          <AlertTriangle className="w-4 h-4" />
          Alerts
        </Link>
      );
    },
  },

  Actions: {
    RunBuild: ({ id }) => {
      const build_id = useDeployment(id)?.info.build_id;
      if (!build_id) return null;
      return <RunBuild id={build_id} />;
    },
    DeployContainer,
    StartOrStopContainer,
    RemoveContainer,
  },

  Page: {},

  Config: ConfigOrLog,

  DangerZone: ({ id }) => (
    <>
      <RenameDeployment id={id} />
      <DeleteDeployment id={id} />
    </>
  ),
};
