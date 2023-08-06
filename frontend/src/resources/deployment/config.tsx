import { Configuration } from "@components/config";
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
import { Settings, Save, History, PlusCircle } from "lucide-react";
import { useState } from "react";
import { useParams } from "react-router-dom";

const ImageTypeSelector = ({
  selected,
  onSelect,
}: {
  selected: Types.DeploymentImage["type"] | undefined;
  onSelect: (serverId: Types.DeploymentImage["type"]) => void;
}) => (
  <Select value={selected} onValueChange={onSelect}>
    <SelectTrigger className="max-w-[400px]">
      <SelectValue placeholder="Select A Server" />
    </SelectTrigger>
    <SelectContent>
      <SelectItem value={"Image"}>Image</SelectItem>
      <SelectItem value={"Build"}>Build</SelectItem>
    </SelectContent>
  </Select>
);

const ServersSelector = ({
  selected,
  onSelect,
}: {
  selected: string | undefined;
  onSelect: (serverId: string) => void;
}) => {
  const servers = useRead("ListServers", {}).data;
  return (
    <Select value={selected} onValueChange={onSelect}>
      <SelectTrigger className="max-w-[400px]">
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
  onSelect: (serverId: string) => void;
}) => {
  const builds = useRead("ListBuilds", {}).data;
  return (
    <Select value={selected} onValueChange={onSelect}>
      <SelectTrigger className="max-w-[400px]">
        <SelectValue placeholder="Select A Server" />
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

const EnvVars = ({
  vars,
  set,
}: {
  vars: Types.EnvironmentVar[] | undefined;
  set: (input: Partial<Types.DeploymentConfig>) => void;
}) => (
  <div className="flex flex-col gap-4 border-b pb-4">
    {vars?.map((variable, i) => (
      <div className="flex justify-between gap-4">
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
      Add
    </Button>
  </div>
);

export const DeploymentConfig = () => {
  const id = useParams().deploymentId;
  const deployment = useRead("GetDeployment", { id }).data;
  const [update, set] = useState<Partial<Types.DeploymentConfig>>({});
  const { mutate, isLoading } = useWrite("UpdateDeployment");

  console.log(deployment?.config);

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
          <Button
            variant="outline"
            intent="success"
            onClick={() => mutate({ config: update, id })}
          >
            <Save className="w-4 h-4" />
          </Button>
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
          image: (image, set) => (
            <div className="flex justify-between items-center border-b pb-4 min-h-[40px] ">
              <div>Image</div>
              <div className="flex gap-4">
                <ImageTypeSelector
                  selected={image?.type}
                  onSelect={(type) =>
                    set({
                      image: {
                        type: type as any,
                        params:
                          type === "Image"
                            ? { image: "" }
                            : {
                                build_id: "",
                                version: { major: 0, minor: 0, patch: 0 },
                              },
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
                    />
                  </div>
                )}
              </div>
            </div>
          ),
          environment: (vars, set) => <EnvVars vars={vars} set={set} />,
        }}
      />
    </Section>
  );
};
