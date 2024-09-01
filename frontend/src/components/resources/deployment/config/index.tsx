import { useRead, useWrite } from "@lib/hooks";
import { Types } from "@komodo/client";
import { ReactNode, useState } from "react";
import {
  AddExtraArgMenu,
  ConfigItem,
  InputList,
} from "@components/config/util";
import { ImageConfig } from "./components/image";
import { RestartModeSelector } from "./components/restart";
import { NetworkModeSelector } from "./components/network";
import { PortsConfig } from "./components/ports";
import { EnvVars } from "./components/environment";
import { VolumesConfig } from "./components/volumes";
import { Config } from "@components/config";
import {
  DefaultTerminationSignal,
  TermSignalLabels,
  TerminationTimeout,
} from "./components/term-signal";
import { LabelsConfig, ServerSelector } from "@components/resources/common";
import { TextUpdateMenu } from "@components/util";
import { Button } from "@ui/button";
import { PlusCircle } from "lucide-react";
import { env_to_text } from "@lib/utils";
import { Link } from "react-router-dom";

export const DeploymentConfig = ({
  id,
  titleOther,
}: {
  id: string;
  titleOther: ReactNode;
}) => {
  const perms = useRead("GetPermissionLevel", {
    target: { type: "Deployment", id },
  }).data;
  const config = useRead("GetDeployment", { deployment: id }).data?.config;
  const global_disabled =
    useRead("GetCoreInfo", {}).data?.ui_write_disabled ?? false;
  const [update, set] = useState<Partial<Types.DeploymentConfig>>({});
  const { mutateAsync } = useWrite("UpdateDeployment");

  if (!config) return null;

  const network = update.network ?? config.network;
  const hide_ports = network === "host" || network === "none";

  const disabled = global_disabled || perms !== Types.PermissionLevel.Write;

  return (
    <Config
      titleOther={titleOther}
      disabled={disabled}
      config={config}
      update={update}
      set={set}
      onSave={async () => {
        await mutateAsync({ id, config: update });
      }}
      components={{
        general: [
          {
            label: "Server Id",
            labelHidden: true,
            components: {
              server_id: (value, set) => (
                <ServerSelector
                  selected={value}
                  set={set}
                  disabled={disabled}
                  align="end"
                />
              ),
            },
          },
          {
            label: "Container",
            components: {
              image: (value, set) => (
                <ImageConfig image={value} set={set} disabled={disabled} />
              ),
              restart: (value, set) => (
                <RestartModeSelector
                  selected={value}
                  set={set}
                  disabled={disabled}
                />
              ),
              network: (value, set) => (
                <NetworkModeSelector
                  server_id={update.server_id ?? config.server_id}
                  selected={value}
                  onSelect={(network) => set({ network })}
                  disabled={disabled}
                />
              ),
              command: (value, set) => (
                <ConfigItem
                  label="Command"
                  description={
                    <div className="flex flex-row flex-wrap">
                      <div>Replace the CMD, or extend the ENTRYPOINT.</div>
                      <Link
                        to="https://docs.docker.com/engine/reference/run/#commands-and-arguments"
                        target="_blank"
                      >
                        <Button variant="link" className="p-0">
                          See docker docs.
                        </Button>
                      </Link>
                    </div>
                  }
                >
                  <TextUpdateMenu
                    title="Update Command"
                    placeholder="Set custom command"
                    value={value}
                    onUpdate={(command) => set({ command })}
                    triggerClassName="min-w-[300px] max-w-[400px]"
                    disabled={disabled}
                  />
                </ConfigItem>
              ),
            },
          },
          {
            label: "Ports",
            description: "Configure the port bindings for the container.",
            hidden: hide_ports,
            contentHidden: (update.ports ?? config.ports)?.length === 0,
            actions: !disabled && (
              <Button
                variant="secondary"
                onClick={() =>
                  set((update) => ({
                    ...update,
                    ports: [
                      ...(update.ports ?? config.ports ?? []),
                      { container: "", local: "" },
                    ],
                  }))
                }
                className="flex items-center gap-2 w-[200px]"
              >
                <PlusCircle className="w-4 h-4" />
                Add Port
              </Button>
            ),
            components: {
              ports: (value, set) => (
                <PortsConfig
                  ports={value ?? []}
                  set={set}
                  disabled={disabled}
                />
              ),
            },
          },
          {
            label: "Volumes",
            description: "Configure the volume bindings for the container.",
            contentHidden: (update.volumes ?? config.volumes)?.length === 0,
            actions: !disabled && (
              <Button
                variant="secondary"
                onClick={() =>
                  set((update) => ({
                    ...update,
                    volumes: [
                      ...(update.volumes ?? config.volumes ?? []),
                      { container: "", local: "" },
                    ],
                  }))
                }
                className="flex items-center gap-2 w-[200px]"
              >
                <PlusCircle className="w-4 h-4" />
                Add Volume
              </Button>
            ),
            components: {
              volumes: (v, set) => (
                <VolumesConfig
                  volumes={v ?? []}
                  set={set}
                  disabled={disabled}
                />
              ),
            },
          },
          {
            label: "Extra Args",
            description: (
              <div className="flex flex-row flex-wrap">
                <div>Pass extra arguments to 'docker run'.</div>
                <Link
                  to="https://docs.docker.com/engine/reference/run/#commands-and-arguments"
                  target="_blank"
                >
                  <Button variant="link" className="p-0">
                    See docker docs.
                  </Button>
                </Link>
              </div>
            ),
            contentHidden:
              (update.extra_args ?? config.extra_args)?.length === 0,
            actions: !disabled && (
              <AddExtraArgMenu
                type="Deployment"
                onSelect={(suggestion) =>
                  set((update) => ({
                    ...update,
                    extra_args: [
                      ...(update.extra_args ?? config.extra_args ?? []),
                      suggestion,
                    ],
                  }))
                }
                disabled={disabled}
              />
            ),
            components: {
              extra_args: (value, set) => (
                <InputList
                  field="extra_args"
                  values={value ?? []}
                  set={set}
                  disabled={disabled}
                  placeholder="--extra-arg=value"
                />
              ),
            },
          },
          {
            label: "Labels",
            description: "Attach --labels to the container.",
            contentHidden: (update.labels ?? config.labels)?.length === 0,
            actions: !disabled && (
              <Button
                variant="secondary"
                onClick={() =>
                  set({
                    ...update,
                    labels: [
                      ...(update.labels ?? config.labels ?? []),
                      { variable: "", value: "" },
                    ] as Types.EnvironmentVar[],
                  })
                }
                className="flex items-center gap-2 w-[200px]"
              >
                <PlusCircle className="w-4 h-4" />
                Add Label
              </Button>
            ),
            components: {
              labels: (l, set) => (
                <LabelsConfig
                  labels={(l as Types.EnvironmentVar[]) ?? []}
                  set={set}
                  disabled={disabled}
                />
              ),
            },
          },
          {
            label: "Settings",
            components: {
              send_alerts: true,
              redeploy_on_build:
                (update.image?.type || config.image?.type) === "Build",
            },
          },
        ],
        environment: [
          {
            label: "Environment",
            description:
              "Pass environment variables to the container. You can interpolate variables and secrets using '[[VAR_NAME]]'",
            components: {
              environment: (vars, set) => {
                const env = typeof vars === "object" ? env_to_text(vars) : vars;
                return (
                  <EnvVars
                    env={env ?? ""}
                    set={set}
                    server={update.server_id || config.server_id}
                    disabled={disabled}
                  />
                );
              },
              skip_secret_interp: true,
            },
          },
        ],
        termination: [
          {
            label: "Termination",
            description:
              "Configure the ways to 'docker stop' the container.",
            components: {
              termination_signal: (value, set) => (
                <DefaultTerminationSignal
                  arg={value}
                  set={set}
                  disabled={disabled}
                />
              ),
              termination_timeout: (value, set) => (
                <TerminationTimeout arg={value} set={set} disabled={disabled} />
              ),
              term_signal_labels: (value, set) => (
                <TermSignalLabels args={value} set={set} disabled={disabled} />
              ),
            },
          },
        ],
      }}
    />
  );
};
