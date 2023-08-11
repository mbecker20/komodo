import { ConfigAgain } from "@components/config/again";
import { useRead, useWrite } from "@hooks";
import { ResourceCard } from "@layouts/card";
import { ConfigLayout } from "@layouts/page";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { Card, CardHeader, CardTitle, CardContent } from "@ui/card";
import { Input } from "@ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";
import { readableVersion } from "@util/helpers";
import { useState } from "react";
import { Link } from "react-router-dom";
import {
  DeploymentStatusIcon,
  DeploymentServer,
  DeploymentBuild,
} from "../util";
import { DoubleInput, ResourceSelector } from "@components/config/util";

const ImageTypeSelector = ({
  selected,
  onSelect,
}: {
  selected: Types.DeploymentImage["type"] | undefined;
  onSelect: (type: Types.DeploymentImage["type"]) => void;
}) => (
  <Select value={selected || undefined} onValueChange={onSelect}>
    <SelectTrigger className="max-w-[150px]">
      <SelectValue placeholder="Select Type" />
    </SelectTrigger>
    <SelectContent>
      <SelectItem value={"Image"}>Image</SelectItem>
      <SelectItem value={"Build"}>Build</SelectItem>
    </SelectContent>
  </Select>
);

const BuildVersionSelector = ({
  buildId,
  selected,
  onSelect,
}: {
  buildId: string | undefined;
  selected: string | undefined;
  onSelect: (version: string) => void;
}) => {
  const versions = useRead("GetBuildVersions", { id: buildId }).data;
  return (
    <Select value={selected || undefined} onValueChange={onSelect}>
      <SelectTrigger className="w-full lg:w-[150px]">
        <SelectValue placeholder="Select Version" />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value={JSON.stringify({ major: 0, minor: 0, patch: 0 })}>
          latest
        </SelectItem>
        {versions?.map((v) => (
          <SelectItem key={JSON.stringify(v)} value={JSON.stringify(v)}>
            {readableVersion(v.version)}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
};

export const EnvVars = ({
  vars,
  set,
}: {
  vars: Types.EnvironmentVar[];
  set: (input: Partial<Types.DeploymentConfig>) => void;
}) => (
  <DoubleInput
    values={vars}
    leftval="variable"
    rightval="value"
    onLeftChange={(variable, i) => {
      vars[i].variable = variable;
      set({ environment: [...vars] });
    }}
    onRightChange={(value, i) => {
      vars[i].value = value;
      set({ environment: [...vars] });
    }}
    onAdd={() =>
      set({ environment: [...(vars ?? []), { variable: "", value: "" }] })
    }
    onRemove={(idx) =>
      set({ environment: [...vars.filter((_, i) => i !== idx)] })
    }
  />
);

export const PortsConfig = ({
  ports,
  set,
}: {
  ports: Types.Conversion[];
  set: (input: Partial<Types.DeploymentConfig>) => void;
}) => (
  <DoubleInput
    values={ports}
    leftval="container"
    rightval="local"
    onLeftChange={(container, i) => {
      ports[i].container = container;
      set({ ports: [...ports] });
    }}
    onRightChange={(local, i) => {
      ports[i].local = local;
      set({ ports: [...ports] });
    }}
    onAdd={() =>
      set({ ports: [...(ports ?? []), { container: "", local: "" }] })
    }
    onRemove={(idx) => set({ ports: [...ports.filter((_, i) => i !== idx)] })}
  />
);

export const ImageConfig = ({
  image,
  set,
}: {
  image: Types.DeploymentImage | undefined;
  set: (input: Partial<Types.DeploymentConfig>) => void;
}) => (
  <div className="flex justify-between items-center border-b pb-4 min-h-[40px]">
    <div>Image</div>
    <div className="flex gap-4 w-full justify-end">
      <ImageTypeSelector
        selected={image?.type}
        onSelect={(type) =>
          set({
            image: {
              type: type as any,
              params:
                type === "Image"
                  ? { image: "" }
                  : ({
                      build_id: "",
                      version: { major: 0, minor: 0, patch: 0 },
                    } as any),
            },
          })
        }
      />
      {image?.type === "Build" && (
        <div className="flex gap-4">
          <ResourceSelector
            type="Build"
            selected={image.params.build_id}
            onSelect={(id) =>
              set({
                image: {
                  ...image,
                  params: { ...image.params, build_id: id },
                },
              })
            }
          />
          <BuildVersionSelector
            buildId={image.params.build_id}
            selected={JSON.stringify(image.params.version)}
            onSelect={(version) =>
              set({
                image: {
                  ...image,
                  params: {
                    ...image.params,
                    version: JSON.parse(version),
                  },
                },
              })
            }
          />
        </div>
      )}
      {image?.type === "Image" && (
        <div>
          <Input
            value={image.params.image}
            onChange={(e) =>
              set({
                image: {
                  ...image,
                  params: { image: e.target.value },
                },
              })
            }
            className="w-full lg:w-[300px]"
            placeholder="image name"
          />
        </div>
      )}
    </div>
  </div>
);

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
                      <ResourceSelector
                        type="Server"
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
                    <PortsConfig ports={value ?? []} set={set} />
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
                  environment: (vars, set) => (
                    <EnvVars vars={vars ?? []} set={set} />
                  ),
                }}
              />
            )}

            {/* Environment Config */}
            {/* {show === "volumes" && (
              <ConfigAgain
                config={config}
                update={update}
                set={(u) => set((p) => ({ ...p, ...u }))}
                components={{
                  volumes: (value, set) => (
                    <PortsConfig ports={value ?? []} set={set} />
                  ),
                }}
              />
            )} */}
          </CardContent>
        </Card>
      </div>
    </ConfigLayout>
  );
};

export const DeploymentConfig = ({ id }: { id: string }) => {
  const config = useRead("GetDeployment", { id }).data?.config;
  if (!config) return null;
  return <DeploymentConfigInner id={id} config={config} />;
};
