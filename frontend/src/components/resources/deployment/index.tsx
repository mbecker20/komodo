import { useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { RequiredResourceComponents } from "@types";
import { AlertTriangle, HardDrive, Rocket } from "lucide-react";
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
import { snake_case_to_upper_space_case } from "@lib/formatting";
import {
  deployment_state_intention,
  fill_color_class_by_intention,
  text_color_class_by_intention,
} from "@lib/color";
import { DeploymentTable } from "./table";
import { ResourceLink } from "@components/util";
import { DeploymentsChart } from "./dashboard";

export const useDeployment = (id?: string) =>
  useRead("ListDeployments", {}, { refetchInterval: 5000 }).data?.find(
    (d) => d.id === id
  );

export const DeploymentComponents: RequiredResourceComponents = {
  Name: ({ id }) => <>{useDeployment(id)?.name}</>,
  Description: ({ id }) => <>{useDeployment(id)?.info.status}</>,
  Link: ({ id }) => <ResourceLink type="Deployment" id={id} />,
  Info: [
    ({ id }) => {
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
    ({ id }) => {
      const info = useDeployment(id)?.info;
      const server = useServer(info?.server_id);
      return server?.id ? (
        <ResourceLink type="Server" id={server?.id} />
      ) : (
        "Unknown Server"
      );
    },
  ],
  Icon: ({ id }) => {
    const state = useDeployment(id)?.info.state;
    const color = fill_color_class_by_intention(
      deployment_state_intention(state)
    );
    return <Rocket className={cn("w-4 h-4", state && color)} />;
  },
  Status: ({ id }) => {
    const state =
      useDeployment(id)?.info.state ?? Types.DockerContainerState.Unknown;
    const color = text_color_class_by_intention(
      deployment_state_intention(state)
    );
    return <div className={color}>{snake_case_to_upper_space_case(state)}</div>;
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
        entityType="Deployment"
        onSuccess={() => mutateAsync({ name, config: {} })}
        enabled={!!name}
      >
        <div className="grid md:grid-cols-2">
          Deployment Name
          <Input
            placeholder="deployment-name"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
        </div>
      </NewResource>
    );
  },
  Table: () => {
    const deployments = useRead("ListDeployments", {}).data;
    return <DeploymentTable deployments={deployments} />;
  },
  Dashboard: DeploymentsChart,
};
