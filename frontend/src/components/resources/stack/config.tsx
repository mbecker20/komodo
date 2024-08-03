import { Config } from "@components/config";
import {
  AccountSelectorConfig,
  AddExtraArgMenu,
  ConfigItem,
  InputList,
  ProviderSelectorConfig,
} from "@components/config/util";
import { useInvalidate, useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { ReactNode, useState } from "react";
import { CopyGithubWebhook, ServerSelector } from "../common";
import { useToast } from "@ui/use-toast";
import { text_color_class_by_intention } from "@lib/color";
import { ConfirmButton } from "@components/util";
import { Ban, CirclePlus } from "lucide-react";

export const StackConfig = ({ id, titleOther }: { id: string; titleOther: ReactNode; }) => {
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
                />
              ),
            },
          },
          {
            label: "General",
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
              repo: { placeholder: "Enter Repo" },
              branch: { placeholder: "Enter branch" },
              commit: {
                placeholder: "Enter a specific commit hash. Optional.",
              },
            },
          },
          {
            label: "Run Path / File",
            components: {
              run_directory: { placeholder: "." },
              file_path: { placeholder: "compose.yaml" },
            },
          },
          {
            label: "Extra Args",
            contentHidden:
              (update.extra_args ?? config.extra_args)?.length === 0,
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
            label: "Github Webhooks",
            components: {
              ["clone" as any]: () => (
                <ConfigItem label="Clone">
                  <CopyGithubWebhook path={`/Stack/${id}/clone`} />
                </ConfigItem>
              ),
              ["pull" as any]: () => (
                <ConfigItem label="Pull">
                  <CopyGithubWebhook path={`/Stack/${id}/pull`} />
                </ConfigItem>
              ),
              webhook_enabled: webhooks !== undefined && !webhooks.managed,
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
                if (!webhooks || !webhooks.managed) return;
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
                            className={text_color_class_by_intention("Neutral")}
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
                            className={text_color_class_by_intention("Neutral")}
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
                    {!webhooks.deploy_enabled && !webhooks.refresh_enabled && (
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
