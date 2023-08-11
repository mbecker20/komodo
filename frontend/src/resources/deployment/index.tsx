import { ConfigAgain } from "@components/config/again";
import { ResourceUpdates } from "@components/updates/resource";
import { useAddRecentlyViewed, useRead, useWrite } from "@hooks";
import { ResourceCard } from "@layouts/card";
import { ConfigLayout } from "@layouts/page";
import { Resource } from "@layouts/resource";
import { Types } from "@monitor/client";
import {
  RedeployContainer,
  StartOrStopContainer,
  RemoveContainer,
} from "@resources/deployment/components/actions";
import { DeploymentLogs } from "@resources/deployment/components/deployment-logs";
import {
  EnvVars,
  ImageConfig,
  PortsConfig,
  ServersSelector,
} from "@resources/deployment/config";
import {
  DeploymentBuild,
  DeploymentName,
  DeploymentServer,
  DeploymentStatus,
  DeploymentStatusIcon,
} from "@resources/deployment/util";
import { Button } from "@ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@ui/card";
import { useState } from "react";
import { Link, useParams } from "react-router-dom";

export const DeploymentCard = ({ id }: { id: string }) => {
  const deployments = useRead("ListDeployments", {}).data;
  const deployment = deployments?.find((d) => d.id === id);
  if (!deployment) return null;
  return (
    <Link to={`/deployments/${deployment.id}`}>
      <ResourceCard
        title={deployment.name}
        description={deployment.info.status ?? "not deployed"}
        statusIcon={<DeploymentStatusIcon deploymentId={id} />}
      >
        <div className="flex flex-col text-muted-foreground text-sm">
          <DeploymentServer deploymentId={id} />
          <DeploymentBuild deploymentId={id} />
        </div>
      </ResourceCard>
    </Link>
  );
};

const DeploymentConfigInner = ({
  id,
  config,
}: {
  id: string;
  config: Types.DeploymentConfig;
}) => {
  const [update, set] = useState<Partial<Types.DeploymentConfig>>({});
  const [show, setShow] = useState("general");
  const { mutate } = useWrite("UpdateDeployment");

  return (
    <ConfigLayout
      content={update}
      onConfirm={() => mutate({ id, config: update })}
      onReset={() => set({})}
    >
      <div className="flex gap-4">
        <div className="flex flex-col gap-4 w-[300px]">
          {["general", "networking", "environment", "volumes"].map((item) => (
            <Button
              variant={show === item ? "secondary" : "outline"}
              onClick={() => setShow(item)}
              className="capitalize"
            >
              {item}
            </Button>
          ))}
        </div>
        <Card className="w-full">
          <CardHeader className="border-b">
            <CardTitle className="capitalize">{show}</CardTitle>
          </CardHeader>
          <CardContent className="flex flex-col gap-4 mt-4">
            {/* General Config */}
            {show === "general" && (
              <ConfigAgain
                config={config}
                update={update}
                set={(u) => set((p) => ({ ...p, ...u }))}
                components={{
                  server_id: (value, set) => (
                    <div className="flex items-center justify-between border-b pb-4">
                      Server
                      <ServersSelector
                        selected={value}
                        onSelect={(server_id) => set({ server_id })}
                      />
                    </div>
                  ),
                  image: (value, set) => (
                    <ImageConfig image={value} set={set} />
                  ),
                  restart: true,
                }}
              />
            )}

            {/* Networking Config */}
            {show === "networking" && (
              <ConfigAgain
                config={config}
                update={update}
                set={(u) => set((p) => ({ ...p, ...u }))}
                components={{
                  network: true,
                  ports: (value, set) => (
                    <PortsConfig ports={value} set={set} />
                  ),
                }}
              />
            )}

            {/* Environment Config */}
            {show === "environment" && (
              <ConfigAgain
                config={config}
                update={update}
                set={(u) => set((p) => ({ ...p, ...u }))}
                components={{
                  skip_secret_interp: true,
                  environment: (value, set) => (
                    <EnvVars vars={value} set={set} />
                  ),
                }}
              />
            )}

            {/* Environment Config */}
            {show === "volumes" && (
              <ConfigAgain
                config={config}
                update={update}
                set={(u) => set((p) => ({ ...p, ...u }))}
                components={{
                  volumes: (value, set) => (
                    <PortsConfig ports={value} set={set} />
                  ),
                }}
              />
            )}
          </CardContent>
        </Card>
      </div>
    </ConfigLayout>
  );
};

const DeploymentConfig = ({ id }: { id: string }) => {
  const config = useRead("GetDeployment", { id }).data?.config;
  if (!config) return null;
  return <DeploymentConfigInner id={id} config={config} />;
};

export const DeploymentPage = () => {
  const id = useParams().deploymentId;
  if (!id) return null;
  useAddRecentlyViewed("Deployment", id);

  return (
    <Resource
      title={<DeploymentName deploymentId={id} />}
      info={
        <div className="flex flex-col lg:flex-row lg:items-center lg:gap-4 text-muted-foreground">
          <div className="flex items-center gap-2 ">
            <DeploymentStatusIcon deploymentId={id} />
            <DeploymentStatus deploymentId={id} />
          </div>
          <CardDescription className="hidden lg:block">|</CardDescription>
          <DeploymentServer deploymentId={id} />
          <CardDescription className="hidden lg:block">|</CardDescription>
          <DeploymentBuild deploymentId={id} />
        </div>
      }
      actions={
        <div className="flex gap-4">
          <RedeployContainer deployment_id={id} />
          <StartOrStopContainer deployment_id={id} />
          <RemoveContainer deployment_id={id} />
        </div>
      }
    >
      <ResourceUpdates type="Deployment" id={id} />
      <DeploymentLogs deployment_id={id} />
      <DeploymentConfig id={id} />
    </Resource>
  );
};
