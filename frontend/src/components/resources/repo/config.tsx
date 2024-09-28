import { Config } from "@components/config";
import {
  AccountSelectorConfig,
  ConfigItem,
  InputList,
  ProviderSelectorConfig,
  SystemCommand,
} from "@components/config/util";
import { useInvalidate, useRead, useWrite } from "@lib/hooks";
import { Types } from "@komodo/client";
import { useState } from "react";
import { BuilderSelector, CopyGithubWebhook, ServerSelector } from "../common";
import { useToast } from "@ui/use-toast";
import { text_color_class_by_intention } from "@lib/color";
import { ConfirmButton } from "@components/util";
import { Ban, CirclePlus, PlusCircle } from "lucide-react";
import { Button } from "@ui/button";
import { SecretsSearch } from "@components/config/env_vars";
import { MonacoEditor } from "@components/monaco";

export const RepoConfig = ({ id }: { id: string }) => {
  const perms = useRead("GetPermissionLevel", {
    target: { type: "Repo", id },
  }).data;
  const config = useRead("GetRepo", { repo: id }).data?.config;
  const webhooks = useRead("GetRepoWebhooksEnabled", { repo: id }).data;
  const global_disabled =
    useRead("GetCoreInfo", {}).data?.ui_write_disabled ?? false;
  const [update, set] = useState<Partial<Types.RepoConfig>>({});
  const { mutateAsync } = useWrite("UpdateRepo");
  if (!config) return null;

  const disabled = global_disabled || perms !== Types.PermissionLevel.Write;

  return (
    <Config
      resource_id={id}
      resource_type="Repo"
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
                />
              ),
            },
          },
          {
            label: "Builder Id",
            labelHidden: true,
            components: {
              builder_id: (value, set) => (
                <BuilderSelector
                  selected={value}
                  set={set}
                  disabled={disabled}
                />
              ),
            },
          },
          {
            label: "General",
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
              path: {
                label: "Clone Path",
                placeholder: "/clone/path/on/host",
                description:
                  "Explicitly specify the folder on the host to clone the repo in. Optional.",
              },
            },
          },
          {
            label: "Environment",
            description:
              "Write these variables to a .env-formatted file at the specified path, before on_clone / on_pull are run.",
            labelExtra: !disabled && <SecretsSearch />,
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
            label: "On Clone",
            description:
              "Execute a shell command after cloning the repo. The given Cwd is relative to repo root.",
            components: {
              on_clone: (value, set) => (
                <SystemCommand
                  value={value}
                  set={(value) => set({ on_clone: value })}
                  disabled={disabled}
                />
              ),
            },
          },
          {
            label: "On Pull",
            description:
              "Execute a shell command after pulling the repo. The given Cwd is relative to repo root.",
            components: {
              on_pull: (value, set) => (
                <SystemCommand
                  value={value}
                  set={(value) => set({ on_pull: value })}
                  disabled={disabled}
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
                    <div>Must configure Branch before webhooks will work.</div>
                  </ConfigItem>
                );
              },
              ["pull" as any]: () => (
                <ConfigItem label="Pull">
                  <CopyGithubWebhook path={`/repo/${id}/pull`} />
                </ConfigItem>
              ),
              ["clone" as any]: () => (
                <ConfigItem label="Clone">
                  <CopyGithubWebhook path={`/repo/${id}/clone`} />
                </ConfigItem>
              ),
              ["build" as any]: () => (
                <ConfigItem label="Build">
                  <CopyGithubWebhook path={`/repo/${id}/build`} />
                </ConfigItem>
              ),
              webhook_enabled: webhooks !== undefined && !webhooks.managed,
              webhook_secret: {
                description:
                  "Provide a custom webhook secret for this resource, or use the global default.",
                placeholder: "Input custom secret",
              },
              ["managed" as any]: () => {
                const inv = useInvalidate();
                const { toast } = useToast();
                const { mutate: createWebhook, isPending: createPending } =
                  useWrite("CreateRepoWebhook", {
                    onSuccess: () => {
                      toast({ title: "Webhook Created" });
                      inv(["GetRepoWebhooksEnabled", { repo: id }]);
                    },
                  });
                const { mutate: deleteWebhook, isPending: deletePending } =
                  useWrite("DeleteRepoWebhook", {
                    onSuccess: () => {
                      toast({ title: "Webhook Deleted" });
                      inv(["GetRepoWebhooksEnabled", { repo: id }]);
                    },
                  });
                if (!webhooks || !webhooks.managed) return;
                return (
                  <ConfigItem label="Manage Webhook">
                    {webhooks.clone_enabled && (
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
                            className={text_color_class_by_intention("Neutral")}
                          >
                            CLONE
                          </div>
                        </div>
                        <ConfirmButton
                          title="Disable"
                          icon={<Ban className="w-4 h-4" />}
                          variant="destructive"
                          onClick={() =>
                            deleteWebhook({
                              repo: id,
                              action: Types.RepoWebhookAction.Clone,
                            })
                          }
                          loading={deletePending}
                          disabled={disabled || deletePending}
                        />
                      </div>
                    )}
                    {!webhooks.clone_enabled && webhooks.pull_enabled && (
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
                            className={text_color_class_by_intention("Neutral")}
                          >
                            PULL
                          </div>
                        </div>
                        <ConfirmButton
                          title="Disable"
                          icon={<Ban className="w-4 h-4" />}
                          variant="destructive"
                          onClick={() =>
                            deleteWebhook({
                              repo: id,
                              action: Types.RepoWebhookAction.Pull,
                            })
                          }
                          loading={deletePending}
                          disabled={disabled || deletePending}
                        />
                      </div>
                    )}
                    {webhooks.build_enabled && (
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
                            className={text_color_class_by_intention("Neutral")}
                          >
                            BUILD
                          </div>
                        </div>
                        <ConfirmButton
                          title="Disable"
                          icon={<Ban className="w-4 h-4" />}
                          variant="destructive"
                          onClick={() =>
                            deleteWebhook({
                              repo: id,
                              action: Types.RepoWebhookAction.Build,
                            })
                          }
                          loading={deletePending}
                          disabled={disabled || deletePending}
                        />
                      </div>
                    )}
                    {!webhooks.clone_enabled &&
                      !webhooks.pull_enabled &&
                      !webhooks.build_enabled && (
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
                          {(update.server_id ?? config.server_id) && (
                            <ConfirmButton
                              title="Enable Clone"
                              icon={<CirclePlus className="w-4 h-4" />}
                              onClick={() =>
                                createWebhook({
                                  repo: id,
                                  action: Types.RepoWebhookAction.Clone,
                                })
                              }
                              loading={createPending}
                              disabled={disabled || createPending}
                            />
                          )}
                          {(update.server_id ?? config.server_id) && (
                            <ConfirmButton
                              title="Enable Pull"
                              icon={<CirclePlus className="w-4 h-4" />}
                              onClick={() =>
                                createWebhook({
                                  repo: id,
                                  action: Types.RepoWebhookAction.Pull,
                                })
                              }
                              loading={createPending}
                              disabled={disabled || createPending}
                            />
                          )}
                          {(update.builder_id ?? config.builder_id) && (
                            <ConfirmButton
                              title="Enable Build"
                              icon={<CirclePlus className="w-4 h-4" />}
                              onClick={() =>
                                createWebhook({
                                  repo: id,
                                  action: Types.RepoWebhookAction.Build,
                                })
                              }
                              loading={createPending}
                              disabled={disabled || createPending}
                            />
                          )}
                        </div>
                      )}
                  </ConfigItem>
                );
              },
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
      }}
    />
  );
};
