import { useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { RequiredResourceComponents } from "@types";
import { AlertTriangle, Rocket, Server } from "lucide-react";
import { cn } from "@lib/utils";
import { useState } from "react";
import { NewResource, Section } from "@components/layouts";

import { useServer } from "../server";
import { DeploymentConfig } from "./config";
import {
  RedeployContainer,
  StartOrStopContainer,
  RemoveContainer,
  DeleteDeployment,
  RenameDeployment,
} from "./actions";
import { Input } from "@ui/input";
import { DeploymentLogs } from "./logs";

export const useDeployment = (id?: string) =>
  useRead("ListDeployments", {}, { refetchInterval: 5000 }).data?.find(
    (d) => d.id === id
  );

export const Deployment: RequiredResourceComponents = {
  Name: ({ id }) => <>{useDeployment(id)?.name}</>,
  Description: ({ id }) => (
    <>{useDeployment(id)?.info.status ?? "Not Deployed"}</>
  ),
  Info: ({ id }) => (
    <div className="flex items-center gap-2">
      <Server className="w-4 h-4" />
      {useServer(useDeployment(id)?.info.server_id)?.name ?? "N/A"}
    </div>
  ),
  Icon: ({ id }) => {
    const s = useDeployment(id)?.info.state;

    const color = () => {
      if (s === Types.DockerContainerState.Running) return "fill-green-500";
      if (s === Types.DockerContainerState.Paused) return "fill-orange-500";
      if (s === Types.DockerContainerState.NotDeployed) return "fill-blue-500";
      return "fill-red-500";
    };

    return <Rocket className={cn("w-4 h-4", color())} />;
  },
  Actions: ({ id }) => (
    <div className="flex gap-4">
      <RedeployContainer id={id} />
      <StartOrStopContainer id={id} />
      <RemoveContainer id={id} />
    </div>
  ),
  Page: {
    Logs: ({ id }) => <DeploymentLogs id={id} />,
    Config: ({ id }) => <DeploymentConfig id={id} />,
    Danger: ({ id }) => (
      <Section title="Danger Zone" icon={<AlertTriangle className="w-4 h-4" />}>
        <RenameDeployment id={id} />
        <DeleteDeployment id={id} />
      </Section>
    ),
  },
  New: () => {
    const { mutateAsync } = useWrite("CreateDeployment");
    const [name, setName] = useState("");
    return (
      <NewResource
        type="Deployment"
        onSuccess={() => mutateAsync({ name, config: {} })}
      >
        <div className="flex items-center justify-between">
          <Input
            placeholder="deployment-name"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
        </div>
      </NewResource>
    );
  },
};
