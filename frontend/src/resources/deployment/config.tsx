import { Configuration } from "@components/config";
import { ConfirmUpdate } from "@components/config/confirm-update";
import { useRead, useWrite } from "@hooks";
import { Section } from "@layouts/page";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { Input } from "@ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";
import { readableVersion } from "@util/helpers";
import { Settings, History, PlusCircle } from "lucide-react";
import { useState } from "react";
import { useParams } from "react-router-dom";

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

export const ServersSelector = ({
  selected,
  onSelect,
}: {
  selected: string | undefined;
  onSelect: (serverId: string) => void;
}) => {
  const servers = useRead("ListServers", {}).data;
  return (
    <Select value={selected || undefined} onValueChange={onSelect}>
      <SelectTrigger className="w-full lg:w-[300px]">
        <SelectValue placeholder="Select A Server" />
      </SelectTrigger>
      <SelectContent>
        {servers?.map((s) => (
          <SelectItem key={s.id} value={s.id}>
            {s.name}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
};

const BuildsSelector = ({
  selected,
  onSelect,
}: {
  selected: string | undefined;
  onSelect: (buildId: string) => void;
}) => {
  const builds = useRead("ListBuilds", {}).data;
  return (
    <Select value={selected || undefined} onValueChange={onSelect}>
      <SelectTrigger className="w-full lg:w-[300px]">
        <SelectValue placeholder="Select Build" />
      </SelectTrigger>
      <SelectContent>
        {builds?.map((b) => (
          <SelectItem key={b.id} value={b.id}>
            {b.name}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
};

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
  vars: Types.EnvironmentVar[] | undefined;
  set: (input: Partial<Types.DeploymentConfig>) => void;
}) => (
  <div className="flex flex-col gap-4 border-b pb-4">
    {vars?.map((variable, i) => (
      <div className="flex justify-between gap-4" key={i}>
        <Input
          value={variable.variable}
          placeholder="Variable Name"
          onChange={(e) => {
            vars[i].variable = e.target.value;
            set({ environment: [...vars] });
          }}
        />
        =
        <Input
          value={variable.value}
          placeholder="Variable Value"
          onChange={(e) => {
            vars[i].value = e.target.value;
            set({ environment: [...vars] });
          }}
        />
      </div>
    ))}
    <Button
      variant="outline"
      intent="success"
      className="flex items-center gap-2"
      onClick={() =>
        set({
          environment: [...(vars ?? []), { variable: "", value: "" }],
        })
      }
    >
      <PlusCircle className="w-4 h-4" />
      Add Environment Variable
    </Button>
  </div>
);

export const PortsConfig = ({
  ports,
  set,
}: {
  ports: Types.Conversion[] | undefined;
  set: (input: Partial<Types.DeploymentConfig>) => void;
}) => (
  <div className="flex flex-col gap-4 border-b pb-4">
    {ports?.map((port, i) => (
      <div className="flex items-center justify-between gap-4" key={i}>
        <Input
          value={port.container}
          placeholder="Container"
          onChange={(e) => {
            ports[i].container = e.target.value;
            set({ ports: [...ports] });
          }}
        />
        =
        <Input
          value={port.local}
          placeholder="Host"
          onChange={(e) => {
            ports[i].local = e.target.value;
            set({ ports: [...ports] });
          }}
        />
      </div>
    ))}
    <Button
      variant="outline"
      intent="success"
      className="flex items-center gap-2"
      onClick={() =>
        set({
          ports: [...(ports ?? []), { container: "", local: "" }],
        })
      }
    >
      <PlusCircle className="w-4 h-4" />
      Add Port
    </Button>
  </div>
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
          <BuildsSelector
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

export const DeploymentConfig = () => {
  const id = useParams().deploymentId;
  const deployment = useRead("GetDeployment", { id }).data;
  const [update, set] = useState<Partial<Types.DeploymentConfig>>({});
  const { mutate, isLoading } = useWrite("UpdateDeployment");

  if (!id || !deployment?.config) return null;

  return (
    <Section
      title="Config"
      icon={<Settings className="w-4 h-4" />}
      actions={
        <div className="flex gap-4">
          <Button variant="outline" intent="warning" onClick={() => set({})}>
            <History className="w-4 h-4" />
          </Button>
          <ConfirmUpdate
            content={JSON.stringify(update, null, 2)}
            onConfirm={() => mutate({ config: update, id })}
          />
        </div>
      }
    >
      <Configuration
        config={deployment.config}
        loading={isLoading}
        update={update}
        set={(input) => set((update) => ({ ...update, ...input }))}
        layout={{
          general: ["server_id", "image", "restart"],
          networking: ["network", "ports"],
          environment: ["environment", "skip_secret_interp"],
          volumes: ["volumes"],
        }}
        overrides={{
          server_id: (value, set) => (
            <div className="flex items-center justify-between border-b pb-4">
              Server
              <ServersSelector
                selected={value}
                onSelect={(server_id) => set({ server_id })}
              />
            </div>
          ),
          ports: (ports, set) => <PortsConfig ports={ports} set={set} />,
          volumes: (volumes, set) => <PortsConfig ports={volumes} set={set} />,
          image: (image, set) => <ImageConfig image={image} set={set} />,
          environment: (vars, set) => <EnvVars vars={vars} set={set} />,
        }}
      />
    </Section>
  );
};
