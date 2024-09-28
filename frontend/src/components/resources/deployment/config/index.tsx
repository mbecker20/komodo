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
import { Config } from "@components/config";
import { ResourceSelector, ServerSelector } from "@components/resources/common";
import { TextUpdateMenu } from "@components/util";
import { Button } from "@ui/button";
import { PlusCircle } from "lucide-react";
import { Link } from "react-router-dom";
import { SecretsSearch } from "@components/config/env_vars";
import { MonacoEditor } from "@components/monaco";
import {
  DefaultTerminationSignal,
  TerminationTimeout,
} from "./components/term-signal";

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
      resource_id={id}
      resource_type="Deployment"
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
            label: "Server",
            contentHidden: true,
            actions: (
              <ResourceSelector
                type="Server"
                selected={update.server_id ?? config.server_id}
                onSelect={(server_id) => set({ server_id })}
                disabled={disabled}
                align="end"
              />
            ),
            components: {},
          },
          {
            label: "Container",
            labelHidden: true,
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
            components: {
              ports: (ports, set) => (
                <MonacoEditor
                  value={ports || "  # 3000:3000\n"}
                  language="yaml"
                  onValueChange={(ports) => set({ ports })}
                  readOnly={disabled}
                />
              ),
            },
          },
          {
            label: "Volumes",
            description: "Configure the volume bindings for the container.",
            components: {
              volumes: (volumes, set) => (
                <MonacoEditor
                  value={volumes || "  # /local/path:/container/path\n"}
                  language="yaml"
                  onValueChange={(volumes) => set({ volumes })}
                  readOnly={disabled}
                />
              ),
            },
          },
          {
            label: "Environment",
            description:
              "Pass environment variables to the container. You can interpolate variables and secrets using '[[VAR_NAME]]'",
            labelExtra: !disabled && (
              <SecretsSearch server={update.server_id ?? config.server_id} />
            ),
            components: {
              environment: (env, set) => (
                <MonacoEditor
                  value={env || "  # VARIABLE: value\n"}
                  onValueChange={(environment) => set({ environment })}
                  language="yaml"
                  readOnly={disabled}
                />
              ),
              // skip_secret_interp: true,
            },
          },
          {
            label: "Labels",
            description: "Attach --labels to the container.",
            components: {
              labels: (labels, set) => (
                <MonacoEditor
                  value={labels || "  # your.docker.label: value\n"}
                  language="yaml"
                  onValueChange={(labels) => set({ labels })}
                  readOnly={disabled}
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
            label: "Links",
            description: "Add quick links in the resource header",
            contentHidden: ((update.links ?? config.links)?.length ?? 0) === 0,
            actions: !disabled && (
              <Button
                variant="secondary"
                onClick={() =>
                  set((update) => ({
                    ...update,
                    links: [...(update.links ?? config.links ?? []), ""],
                  }))
                }
                className="flex items-center gap-2 w-[200px]"
              >
                <PlusCircle className="w-4 h-4" />
                Add Link
              </Button>
            ),
            components: {
              links: (values, set) => (
                <InputList
                  field="links"
                  values={values ?? []}
                  set={set}
                  disabled={disabled}
                  placeholder="Input link"
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
          {
            label: "Termination",
            description:
              "Configure the signals used to 'docker stop' the container. Options are SIGTERM, SIGQUIT, SIGINT, and SIGHUP.",
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
                <MonacoEditor
                  value={value || "  # SIGTERM: your label"}
                  language="yaml"
                  onValueChange={(term_signal_labels) =>
                    set({ term_signal_labels })
                  }
                  readOnly={disabled}
                />
              ),
            },
          },
        ],
      }}
    />
  );
};
