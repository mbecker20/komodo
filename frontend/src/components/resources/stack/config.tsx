import { Config } from "@components/config";
import {
  AccountSelectorConfig,
  AddExtraArgMenu,
  ConfigItem,
  InputList,
  ProviderSelectorConfig,
} from "@components/config/util";
import { useInvalidate, useLocalStorage, useRead, useWrite } from "@lib/hooks";
import { Types } from "@komodo/client";
import { ReactNode, useState } from "react";
import { CopyGithubWebhook, ServerSelector } from "../common";
import { useToast } from "@ui/use-toast";
import { text_color_class_by_intention } from "@lib/color";
import { ConfirmButton, ShowHideButton } from "@components/util";
import { Ban, CirclePlus, PlusCircle } from "lucide-react";
import { Button } from "@ui/button";
import { MonacoEditor } from "@components/monaco";
import { SecretsSearch } from "@components/config/env_vars";

export const StackConfig = ({
  id,
  titleOther,
}: {
  id: string;
  titleOther: ReactNode;
}) => {
  const [show, setShow] = useLocalStorage(`stack-${id}-show`, {
    file: true,
    env: true,
  });
  const perms = useRead("GetPermissionLevel", {
    target: { type: "Stack", id },
  }).data;
  const config = useRead("GetStack", { stack: id }).data?.config;
  const webhooks = useRead("GetStackWebhooksEnabled", { stack: id }).data;
  const global_disabled =
    useRead("GetCoreInfo", {}).data?.ui_write_disabled ?? false;
  const [update, set] = useState<Partial<Types.StackConfig>>({});
  const { mutateAsync } = useWrite("UpdateStack");

  if (!config) return null;

  const disabled = global_disabled || perms !== Types.PermissionLevel.Write;
  const files_on_host = update.files_on_host ?? config.files_on_host;
  const ui_file_contents =
    (update.file_contents ?? config.file_contents ?? "").length > 0;
  const run_build = update.run_build ?? config.run_build;
  const repo_set = update.repo ?? config.repo ? true : false;

  return (
    <Config
      resource_id={id}
      resource_type="Stack"
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
            label: "Compose File",
            hidden: files_on_host || (!ui_file_contents && repo_set),
            description:
              "Manage the file contents here, or use a git repo / files on host option.",
            actions: (
              <ShowHideButton
                show={show.file}
                setShow={(file) => setShow({ ...show, file })}
              />
            ),
            contentHidden: !show.file,
            components: {
              file_contents: (file_contents, set) => {
                const show_default =
                  !file_contents &&
                  update.file_contents === undefined &&
                  !(update.repo ?? config.repo);
                return (
                  <MonacoEditor
                    value={
                      show_default ? DEFAULT_STACK_FILE_CONTENTS : file_contents
                    }
                    onValueChange={(file_contents) => set({ file_contents })}
                    language="yaml"
                  />
                );
              },
            },
          },
          {
            label: "Environment",
            description: "Pass these variables to the compose command",
            labelExtra: (
              <SecretsSearch server={update.server_id ?? config.server_id} />
            ),
            actions: (
              <ShowHideButton
                show={show.env}
                setShow={(env) => setShow({ ...show, env })}
              />
            ),
            contentHidden: !show.env,
            components: {
              environment: (env, set) => (
                <MonacoEditor
                  value={env || "  # VARIABLE: value"}
                  onValueChange={(environment) => set({ environment })}
                  language="yaml"
                  readOnly={disabled}
                />
              ),
              env_file_path: {
                description:
                  "The path to write the file to, relative to the root of the repo.",
                placeholder: ".env",
              },
              // skip_secret_interp: true,
            },
          },
          {
            label: "Settings",
            labelHidden: true,
            components: {
              project_name: {
                placeholder: "Compose project name",
                boldLabel: true,
                description:
                  "Optionally override the compose project name. Can import stacks by matching the existing project name on your host.",
              },
              files_on_host: {
                label: "Files on Server",
                boldLabel: true,
                description:
                  "Manage the compose files on server yourself. Just configure the Run Directory and File Paths to your files.",
              },
              auto_pull: {
                label: "Auto Pull Images",
                boldLabel: true,
                description:
                  "Ensure 'docker compose pull' is run before redeploying the Stack. Otherwise, use 'pull_policy' in docker compose file.",
              },
              run_build: {
                label: "Auto Build Images",
                boldLabel: true,
                description:
                  "Ensure 'docker compose build' is run *before* redeploying the Stack. Otherwise, can use '--build' as an Extra Arg",
              },
            },
          },
          {
            label: "Build Extra Args",
            hidden: !run_build,
            description: "Add extra args inserted after 'docker compose build'",
            contentHidden:
              ((update.build_extra_args ?? config.build_extra_args)?.length ??
                0) === 0,
            actions: !disabled && (
              <AddExtraArgMenu
                type="StackBuild"
                onSelect={(suggestion) =>
                  set((update) => ({
                    ...update,
                    build_extra_args: [
                      ...(update.build_extra_args ??
                        config.build_extra_args ??
                        []),
                      suggestion,
                    ],
                  }))
                }
                disabled={disabled}
              />
            ),
            components: {
              build_extra_args: (value, set) => (
                <InputList
                  field="build_extra_args"
                  values={value ?? []}
                  set={set}
                  disabled={disabled}
                  placeholder="--extra-arg=value"
                />
              ),
            },
          },
          {
            label: "Run Path",
            labelHidden: true,
            hidden: !files_on_host,
            components: {
              run_directory: {
                placeholder: "/path/to/folder",
                description:
                  "Set the cwd when running compose up command. Should usually be the parent folder of the compose files.",
                boldLabel: true,
              },
            },
          },
          {
            label: "File Paths",
            hidden: !files_on_host,
            description:
              "Add files to include using 'docker compose -f'. If empty, uses 'compose.yaml'. Relative to 'Run Directory'.",
            contentHidden:
              (update.file_paths ?? config.file_paths)?.length === 0,
            actions: !disabled && (
              <Button
                variant="secondary"
                onClick={() =>
                  set((update) => ({
                    ...update,
                    file_paths: [
                      ...(update.file_paths ?? config.file_paths ?? []),
                      "",
                    ],
                  }))
                }
                className="flex items-center gap-2 w-[200px]"
              >
                <PlusCircle className="w-4 h-4" />
                Add File
              </Button>
            ),
            components: {
              file_paths: (value, set) => (
                <InputList
                  field="file_paths"
                  values={value ?? []}
                  set={set}
                  disabled={disabled}
                  placeholder="compose.yaml"
                />
              ),
            },
          },
          {
            label: "Extra Args",
            description: "Add extra args inserted after 'docker compose up -d'",
            contentHidden:
              ((update.extra_args ?? config.extra_args)?.length ?? 0) === 0,
            actions: !disabled && (
              <AddExtraArgMenu
                type="Stack"
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
            label: "Image Registry",
            description:
              "Optional. Login to an image registry to pull private images",
            components: {
              registry_provider: (provider, set) => {
                return (
                  <ProviderSelectorConfig
                    account_type="docker"
                    selected={provider}
                    disabled={disabled}
                    onSelect={(registry_provider) => set({ registry_provider })}
                  />
                );
              },
              registry_account: (value, set) => {
                const server_id = update.server_id || config.server_id;
                const provider =
                  update.registry_provider ?? config.registry_provider;
                if (!provider) {
                  return null;
                }
                return (
                  <AccountSelectorConfig
                    id={server_id}
                    type={server_id ? "Server" : "None"}
                    account_type="docker"
                    provider={provider}
                    selected={value}
                    onSelect={(registry_account) => set({ registry_account })}
                    disabled={disabled}
                    placeholder="None"
                  />
                );
              },
            },
          },
          {
            label: "Ignore Services",
            description:
              "If your compose file has init services that exit early, ignore them here so your stack will report the correct health.",
            contentHidden:
              ((update.ignore_services ?? config.ignore_services)?.length ??
                0) === 0,
            actions: !disabled && (
              <Button
                variant="secondary"
                onClick={() =>
                  set((update) => ({
                    ...update,
                    ignore_services: [
                      ...(update.ignore_services ??
                        config.ignore_services ??
                        []),
                      "",
                    ],
                  }))
                }
                className="flex items-center gap-2 w-[200px]"
              >
                <PlusCircle className="w-4 h-4" />
                Add Service
              </Button>
            ),
            components: {
              ignore_services: (values, set) => (
                <InputList
                  field="ignore_services"
                  values={values ?? []}
                  set={set}
                  disabled={disabled}
                  placeholder="Input service name"
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
        ],
        "Git Repo": !files_on_host &&
          !ui_file_contents && [
            {
              label: "Git Repo",
              description:
                "Provide config for repo-based compose files. Not necessary if file contents are configured in UI.",
              components: {
                git_provider: (provider, set) => {
                  const https = update.git_https ?? config.git_https;
                  return (
                    <ProviderSelectorConfig
                      account_type="git"
                      selected={provider}
                      disabled={disabled}
                      onSelect={(git_provider) => set({ git_provider })}
                      https={https}
                      onHttpsSwitch={() => set({ git_https: !https })}
                    />
                  );
                },
                git_account: (value, set) => {
                  const server_id = update.server_id || config.server_id;
                  return (
                    <AccountSelectorConfig
                      id={server_id}
                      type={server_id ? "Server" : "None"}
                      account_type="git"
                      provider={update.git_provider ?? config.git_provider}
                      selected={value}
                      onSelect={(git_account) => set({ git_account })}
                      disabled={disabled}
                      placeholder="None"
                    />
                  );
                },
                repo: {
                  placeholder: "Enter repo",
                  description:
                    "The repo path on the provider. {namespace}/{repo_name}",
                },
                branch: {
                  placeholder: "Enter branch",
                  description: "Select a custom branch, or default to 'main'.",
                },
                commit: {
                  placeholder: "Enter a specific commit hash. Optional.",
                  description:
                    "Switch to a specific hash after cloning the branch.",
                },
              },
            },
            {
              label: "Run Path",
              labelHidden: true,
              components: {
                run_directory: {
                  placeholder: "./",
                  description:
                    "Set the cwd when running compose up command. Relative to the repo root.",
                  boldLabel: true,
                },
              },
            },
            {
              label: "File Paths",
              description:
                "Add files to include using 'docker compose -f'. If empty, uses 'compose.yaml'. Relative to 'Run Directory'.",
              contentHidden:
                (update.file_paths ?? config.file_paths)?.length === 0,
              actions: !disabled && (
                <Button
                  variant="secondary"
                  onClick={() =>
                    set((update) => ({
                      ...update,
                      file_paths: [
                        ...(update.file_paths ?? config.file_paths ?? []),
                        "",
                      ],
                    }))
                  }
                  className="flex items-center gap-2 w-[200px]"
                >
                  <PlusCircle className="w-4 h-4" />
                  Add File
                </Button>
              ),
              components: {
                file_paths: (value, set) => (
                  <InputList
                    field="file_paths"
                    values={value ?? []}
                    set={set}
                    disabled={disabled}
                    placeholder="compose.yaml"
                  />
                ),
              },
            },
            {
              label: "Git Webhooks",
              description:
                "Configure your repo provider to send webhooks to Komodo",
              components: {
                ["Guard" as any]: () => {
                  if (update.branch ?? config.branch) {
                    return null;
                  }
                  return (
                    <ConfigItem label="Configure Branch">
                      <div>
                        Must configure Branch before webhooks will work.
                      </div>
                    </ConfigItem>
                  );
                },
                ["Refresh" as any]: () =>
                  (update.branch ?? config.branch) && (
                    <ConfigItem label="Refresh Cache">
                      <CopyGithubWebhook path={`/stack/${id}/refresh`} />
                    </ConfigItem>
                  ),
                ["Deploy" as any]: () =>
                  (update.branch ?? config.branch) && (
                    <ConfigItem label="Auto Redeploy">
                      <CopyGithubWebhook path={`/stack/${id}/deploy`} />
                    </ConfigItem>
                  ),
                webhook_enabled:
                  !!(update.branch ?? config.branch) &&
                  webhooks !== undefined &&
                  !webhooks.managed,
                webhook_secret: {
                  description:
                    "Provide a custom webhook secret for this resource, or use the global default.",
                  placeholder: "Input custom secret",
                },
                ["managed" as any]: () => {
                  const inv = useInvalidate();
                  const { toast } = useToast();
                  const { mutate: createWebhook, isPending: createPending } =
                    useWrite("CreateStackWebhook", {
                      onSuccess: () => {
                        toast({ title: "Webhook Created" });
                        inv(["GetStackWebhooksEnabled", { stack: id }]);
                      },
                    });
                  const { mutate: deleteWebhook, isPending: deletePending } =
                    useWrite("DeleteStackWebhook", {
                      onSuccess: () => {
                        toast({ title: "Webhook Deleted" });
                        inv(["GetStackWebhooksEnabled", { stack: id }]);
                      },
                    });

                  if (
                    !(update.branch ?? config.branch) ||
                    !webhooks ||
                    !webhooks.managed
                  ) {
                    return null;
                  }

                  return (
                    <ConfigItem label="Manage Webhook">
                      {webhooks.deploy_enabled && (
                        <div className="flex items-center gap-4 flex-wrap">
                          <div className="flex items-center gap-2">
                            Incoming webhook is{" "}
                            <div
                              className={text_color_class_by_intention("Good")}
                            >
                              ENABLED
                            </div>
                            and will trigger
                            <div
                              className={text_color_class_by_intention(
                                "Neutral"
                              )}
                            >
                              DEPLOY
                            </div>
                          </div>
                          <ConfirmButton
                            title="Disable"
                            icon={<Ban className="w-4 h-4" />}
                            variant="destructive"
                            onClick={() =>
                              deleteWebhook({
                                stack: id,
                                action: Types.StackWebhookAction.Deploy,
                              })
                            }
                            loading={deletePending}
                            disabled={disabled || deletePending}
                          />
                        </div>
                      )}
                      {!webhooks.deploy_enabled && webhooks.refresh_enabled && (
                        <div className="flex items-center gap-4 flex-wrap">
                          <div className="flex items-center gap-2">
                            Incoming webhook is{" "}
                            <div
                              className={text_color_class_by_intention("Good")}
                            >
                              ENABLED
                            </div>
                            and will trigger
                            <div
                              className={text_color_class_by_intention(
                                "Neutral"
                              )}
                            >
                              REFRESH
                            </div>
                          </div>
                          <ConfirmButton
                            title="Disable"
                            icon={<Ban className="w-4 h-4" />}
                            variant="destructive"
                            onClick={() =>
                              deleteWebhook({
                                stack: id,
                                action: Types.StackWebhookAction.Refresh,
                              })
                            }
                            loading={deletePending}
                            disabled={disabled || deletePending}
                          />
                        </div>
                      )}
                      {!webhooks.deploy_enabled &&
                        !webhooks.refresh_enabled && (
                          <div className="flex items-center gap-4 flex-wrap">
                            <div className="flex items-center gap-2">
                              Incoming webhook is{" "}
                              <div
                                className={text_color_class_by_intention(
                                  "Critical"
                                )}
                              >
                                DISABLED
                              </div>
                            </div>
                            <ConfirmButton
                              title="Enable Deploy"
                              icon={<CirclePlus className="w-4 h-4" />}
                              onClick={() =>
                                createWebhook({
                                  stack: id,
                                  action: Types.StackWebhookAction.Deploy,
                                })
                              }
                              loading={createPending}
                              disabled={disabled || createPending}
                            />
                            <ConfirmButton
                              title="Enable Refresh"
                              icon={<CirclePlus className="w-4 h-4" />}
                              onClick={() =>
                                createWebhook({
                                  stack: id,
                                  action: Types.StackWebhookAction.Refresh,
                                })
                              }
                              loading={createPending}
                              disabled={disabled || createPending}
                            />
                          </div>
                        )}
                    </ConfigItem>
                  );
                },
              },
            },
          ],
      }}
    />
  );
};

const DEFAULT_STACK_FILE_CONTENTS = `## 🦎 Hello Komodo 🦎
services:
  hello_world:
    image: hello-world
    # networks:
    #   - default
    # ports:
    #   - 3000:3000
    # volumes:
    #   - data:/data

# networks:
#   default: {}

# volumes:
#   data:
`;
