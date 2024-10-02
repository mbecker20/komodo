import { useRead, useWrite } from "@lib/hooks";
import { Types } from "@komodo/client";
import { ReactNode, useState } from "react";
import {
  AddExtraArgMenu,
  ConfigItem,
  ConfigList,
  InputList,
} from "@components/config/util";
import { ImageConfig } from "./components/image";
import { RestartModeSelector } from "./components/restart";
import { NetworkModeSelector } from "./components/network";
import { Config } from "@components/config";
import { ResourceLink, ResourceSelector } from "@components/resources/common";
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
        "": [
          {
            label: "Server",
            labelHidden: true,
            components: {
              server_id: (server_id, set) => {
                return (
                  <ConfigItem
                    label={
                      server_id ? (
                        <div className="flex gap-3 text-lg">
                          Server:
                          <ResourceLink type="Server" id={server_id} />
                        </div>
                      ) : (
                        "Select Server"
                      )
                    }
                    description="Select the Server to deploy on."
                  >
                    <ResourceSelector
                      type="Server"
                      selected={server_id}
                      onSelect={(server_id) => set({ server_id })}
                      disabled={disabled}
                      align="start"
                    />
                  </ConfigItem>
                );
              },
            },
          },
          {
            label:
              (update.image ?? config.image)?.type === "Build"
                ? "Build"
                : "Image",
            description:
              "Either pass a docker image directly, or choose a Build to deploy",
            boldLabel: false,
            components: {
              image: (value, set) => (
                <ImageConfig image={value} set={set} disabled={disabled} />
              ),
              redeploy_on_build: (update.image?.type ?? config.image?.type) ===
                "Build" && {
                description: "Automatically redeploy when the image is built.",
              },
            },
          },
          {
            label: "Network",
            labelHidden: true,
            components: {
              network: (value, set) => (
                <NetworkModeSelector
                  server_id={update.server_id ?? config.server_id}
                  selected={value}
                  onSelect={(network) => set({ network })}
                  disabled={disabled}
                />
              ),
              ports:
                !hide_ports &&
                ((ports, set) => (
                  <ConfigItem
                    label="Ports"
                    description="Configure port mappings."
                  >
                    <MonacoEditor
                      value={ports || "  # 3000:3000\n"}
                      language="key_value"
                      onValueChange={(ports) => set({ ports })}
                      readOnly={disabled}
                    />
                  </ConfigItem>
                )),
            },
          },
          {
            label: "Links",
            labelHidden: true,
            components: {
              links: (values, set) => (
                <ConfigList
                  label="Links"
                  description="Add quick links in the resource header"
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
            label: "Environment",
            boldLabel: false,
            description: "Pass these variables to the container",
            // actions: (
            //   <ShowHideButton
            //     show={show.env}
            //     setShow={(env) => setShow({ ...show, env })}
            //   />
            // ),
            // contentHidden: !show.env,
            components: {
              environment: (env, set) => (
                <div className="flex flex-col gap-4">
                  <SecretsSearch
                    server={update.server_id ?? config.server_id}
                  />
                  <MonacoEditor
                    value={env || "  # VARIABLE = value\n"}
                    onValueChange={(environment) => set({ environment })}
                    language="key_value"
                    readOnly={disabled}
                  />
                </div>
              ),
              // skip_secret_interp: true,
            },
          },
          {
            label: "Volumes",
            description: "Configure the volume bindings.",
            boldLabel: false,
            components: {
              volumes: (volumes, set) => (
                <MonacoEditor
                  value={volumes || "  # volume:/container/path\n"}
                  language="key_value"
                  onValueChange={(volumes) => set({ volumes })}
                  readOnly={disabled}
                />
              ),
            },
          },
          {
            label: "Labels",
            description: "Attach --labels to the container.",
            boldLabel: false,
            components: {
              labels: (labels, set) => (
                <MonacoEditor
                  value={labels || "  # your.docker.label: value\n"}
                  language="key_value"
                  onValueChange={(labels) => set({ labels })}
                  readOnly={disabled}
                />
              ),
            },
          },
          {
            label: "Restart",
            labelHidden: true,
            components: {
              restart: (value, set) => (
                <RestartModeSelector
                  selected={value}
                  set={set}
                  disabled={disabled}
                />
              ),
            },
          },
          {
            label: "Extra Args",
            labelHidden: true,
            components: {
              extra_args: (value, set) => (
                <ConfigItem
                  label="Extra Args"
                  description={
                    <div className="flex flex-row flex-wrap gap-2">
                      <div>Pass extra arguments to 'docker run'.</div>
                      <Link
                        to="https://docs.docker.com/engine/reference/run/#commands-and-arguments"
                        target="_blank"
                        className="text-primary"
                      >
                        See docker docs.
                      </Link>
                    </div>
                  }
                >
                  {!disabled && (
                    <AddExtraArgMenu
                      type="Deployment"
                      onSelect={(suggestion) =>
                        set({
                          extra_args: [
                            ...(update.extra_args ?? config.extra_args ?? []),
                            suggestion,
                          ],
                        })
                      }
                      disabled={disabled}
                    />
                  )}
                  <InputList
                    field="extra_args"
                    values={value ?? []}
                    set={set}
                    disabled={disabled}
                    placeholder="--extra-arg=value"
                  />
                </ConfigItem>
              ),
            },
          },
          {
            label: "Command",
            labelHidden: true,
            components: {
              command: (value, set) => (
                <ConfigItem
                  label="Command"
                  description={
                    <div className="flex flex-row flex-wrap gap-2">
                      <div>Replace the CMD, or extend the ENTRYPOINT.</div>
                      <Link
                        to="https://docs.docker.com/engine/reference/run/#commands-and-arguments"
                        target="_blank"
                        className="text-primary"
                      >
                        See docker docs.
                        {/* <Button variant="link" className="p-0">
                        </Button> */}
                      </Link>
                    </div>
                  }
                >
                  <MonacoEditor
                    value={value}
                    language={undefined}
                    onValueChange={(command) => set({ command })}
                    readOnly={disabled}
                  />
                </ConfigItem>
              ),
            },
          },
          {
            label: "Termination",
            boldLabel: false,
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
                <ConfigItem
                  label="Termination Signal Labels"
                  description="Choose between multiple signals when stopping"
                >
                  <MonacoEditor
                    value={value || "  # SIGTERM: your label\n"}
                    language="key_value"
                    onValueChange={(term_signal_labels) =>
                      set({ term_signal_labels })
                    }
                    readOnly={disabled}
                  />
                </ConfigItem>
              ),
            },
          },
        ],
      }}
    />
  );
};
