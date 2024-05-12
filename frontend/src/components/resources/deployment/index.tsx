import { useRead } from "@lib/hooks";
import { Types } from "@monitor/client";
import { RequiredResourceComponents } from "@types";
import { HardDrive, Rocket } from "lucide-react";
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
import { snake_case_to_upper_space_case } from "@lib/formatting";
import {
  bg_color_class_by_intention,
  deployment_state_intention,
  fill_color_class_by_intention,
} from "@lib/color";
import { DeploymentTable } from "./table";
import { DeploymentsChart } from "./dashboard";
import { NewResource, ResourceLink } from "../common";
import { Card, CardHeader } from "@ui/card";
import { RunBuild } from "../build/actions";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@ui/tabs";
import { DeploymentConfig } from "./config";
import { atomWithStorage } from "jotai/utils";
import { useAtom } from "jotai";

const configOrLog = atomWithStorage("config-or-log-v1", "Config");

export const useDeployment = (id?: string) =>
  useRead("ListDeployments", {}, { refetchInterval: 5000 }).data?.find(
    (d) => d.id === id
  );

const ConfigOrLog = ({ id }: { id: string }) => {
  const [view, setView] = useAtom(configOrLog);
  const state = useDeployment(id)?.info.state;
  const logsDisabled =
    state === undefined ||
    state === Types.DockerContainerState.Unknown ||
    state === Types.DockerContainerState.NotDeployed;
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
              <TabsTrigger value="Config" className="w-[70px]">
                Config
              </TabsTrigger>
              <TabsTrigger
                value="Log"
                className="w-[70px]"
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
              <TabsTrigger value="Config" className="w-[70px]">
                Config
              </TabsTrigger>
              <TabsTrigger
                value="Log"
                className="w-[70px]"
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
  const color = fill_color_class_by_intention(
    deployment_state_intention(state)
  );
  return <Rocket className={cn(`w-${size} h-${size}`, state && color)} />;
};

export const DeploymentComponents: RequiredResourceComponents = {
  Dashboard: DeploymentsChart,

  New: () => <NewResource type="Deployment" />,

  Table: ({ search }) => {
    const deployments = useRead("ListDeployments", {}).data;
    return <DeploymentTable deployments={deployments} search={search} />;
  },

  Name: ({ id }) => <>{useDeployment(id)?.name}</>,
  name: (id) => useDeployment(id)?.name,

  Icon: ({ id }) => <DeploymentIcon id={id} size={4} />,
  BigIcon: ({ id }) => <DeploymentIcon id={id} size={8} />,

  Status: {
    State: ({ id }) => {
      const state =
        useDeployment(id)?.info.state ?? Types.DockerContainerState.Unknown;
      const color = bg_color_class_by_intention(
        deployment_state_intention(state)
      );
      return (
        <Card className={cn("w-fit", color)}>
          <CardHeader className="py-0 px-2">
            {snake_case_to_upper_space_case(state)}
          </CardHeader>
        </Card>
      );
    },
    Status: ({ id }) => {
      const status = useDeployment(id)?.info.status;
      return status && <div>{status}</div>;
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
          {info?.image || "N/A"}
        </div>
      );
    },
    Server: ({ id }) => {
      const info = useDeployment(id)?.info;
      const server = useServer(info?.server_id);
      return server?.id ? (
        <ResourceLink type="Server" id={server?.id} />
      ) : (
        "Unknown Server"
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
