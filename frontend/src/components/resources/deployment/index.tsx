import { useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { RequiredResourceComponents } from "@types";
import { AlertTriangle, HardDrive, Rocket, Server } from "lucide-react";
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
import { Link } from "react-router-dom";
import { DataTable } from "@ui/data-table";
import { ResourceComponents } from "..";
import { TagsWithBadge, useTagsFilter } from "@components/tags";
import { DeploymentsChart } from "@components/dashboard/deployments-chart";
import { Button } from "@ui/button";
import { snake_case_to_upper_space_case } from "@lib/formatting";
import {
  deployment_state_intention,
  fill_color_class_by_intention,
  text_color_class_by_intention,
} from "@lib/color";

export const useDeployment = (id?: string) =>
  useRead("ListDeployments", {}, { refetchInterval: 5000 }).data?.find(
    (d) => d.id === id
  );

const Icon = ({ id }: { id?: string }) => {
  const state = useDeployment(id)?.info.state;

  return (
    <Rocket
      className={cn(
        "w-4",
        fill_color_class_by_intention(deployment_state_intention(state))
      )}
    />
  );
};

const Name = ({ id }: { id: string }) => <>{useDeployment(id)?.name}</>;
const Description = ({ id }: { id: string }) => (
  <>{useDeployment(id)?.info.status}</>
);

const Info = ({ id }: { id: string }) => {
  const info = useDeployment(id)?.info;
  const server = useServer(info?.server_id);

  return (
    <>
      <Link
        to={info?.build_id ? `/builds/${info.build_id}` : "."}
        className="flex items-center gap-2"
      >
        <HardDrive className="w-4 h-4" />
        {useDeployment(id)?.info.image || "N/A"}
      </Link>
      <Link to={`/servers/${server?.id}`}>
        <Button variant="link" className="flex items-center gap-2">
          <Server className="w-4 h-4" />
          {server?.name ?? "N/A"}
        </Button>
      </Link>
    </>
  );
};

export const DeploymentTable = ({
  deployments,
}: {
  deployments: Types.DeploymentListItem[] | undefined;
}) => {
  const tags = useTagsFilter();
  return (
    <DataTable
      data={
        deployments?.filter((deployment) =>
          tags.every((tag) => deployment.tags.includes(tag))
        ) ?? []
      }
      columns={[
        {
          header: "Name",
          cell: ({ row }) => {
            const id = row.original.id;
            return (
              <Link to={`/deployments/${id}`}>
                <Button variant="link" className="flex gap-2 items-center p-0">
                  <ResourceComponents.Deployment.Icon id={id} />
                  <ResourceComponents.Deployment.Name id={id} />
                </Button>
              </Link>
            );
          },
        },
        {
          header: "Image",
          cell: ({
            row: {
              original: {
                info: { build_id, image },
              },
            },
          }) => {
            const builds = useRead("ListBuilds", {}).data;
            if (build_id) {
              const build = builds?.find((build) => build.id === build_id);
              if (build) {
                return (
                  <Link to={`/builds/${build_id}`}>
                    <Button
                      variant="link"
                      className="flex gap-2 items-center p-0"
                    >
                      <ResourceComponents.Build.Icon id={build_id} />
                      <ResourceComponents.Build.Name id={build_id} />
                    </Button>
                  </Link>
                );
              } else {
                return undefined;
              }
            } else {
              const [img, _] = image.split(":");
              return img;
            }
          },
        },
        {
          header: "Server",
          cell: ({ row }) => {
            const id = row.original.info.server_id;
            return (
              <Link to={`/servers/${id}`}>
                <Button variant="link" className="flex items-center gap-2 p-0">
                  <ResourceComponents.Server.Icon id={id} />
                  <ResourceComponents.Server.Name id={id} />
                </Button>
              </Link>
            );
          },
        },
        {
          header: "State",
          cell: ({ row }) => {
            const state = row.original.info.state;
            const color = text_color_class_by_intention(
              deployment_state_intention(state)
            );
            return (
              <div className={color}>
                {snake_case_to_upper_space_case(state)}
              </div>
            );
          },
        },
        {
          header: "Tags",
          cell: ({ row }) => {
            return (
              <div className="flex gap-1">
                <TagsWithBadge tag_ids={row.original.tags} />
              </div>
            );
          },
        },
      ]}
    />
  );
};

export const DeploymentComponents: RequiredResourceComponents = {
  Name,
  Description,
  Info,
  Icon,
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
