import { Config } from "@components/config";
import {
  AccountSelectorConfig,
  ConfigItem,
  ProviderSelectorConfig,
} from "@components/config/util";
import { useInvalidate, useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { ReactNode, useState } from "react";
import { CopyGithubWebhook } from "../common";
import { useToast } from "@ui/use-toast";
import { text_color_class_by_intention } from "@lib/color";
import { ConfirmButton } from "@components/util";
import { Ban, CirclePlus } from "lucide-react";

export const ResourceSyncConfig = ({
  id,
  titleOther,
}: {
  id: string;
  titleOther: ReactNode;
}) => {
  const perms = useRead("GetPermissionLevel", {
    target: { type: "ResourceSync", id },
  }).data;
  const config = useRead("GetResourceSync", { sync: id }).data?.config;
  const webhooks = useRead("GetSyncWebhooksEnabled", { sync: id }).data;
  const global_disabled =
    useRead("GetCoreInfo", {}).data?.ui_write_disabled ?? false;
  const [update, set] = useState<Partial<Types.ResourceSyncConfig>>({});
  const { mutateAsync } = useWrite("UpdateResourceSync");
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
            label: "General",
            components: {
              git_provider: (provider, set) => (
                <ProviderSelectorConfig
                  account_type="git"
                  selected={provider}
                  disabled={disabled}
                  onSelect={(git_provider) => set({ git_provider })}
                />
              ),
              git_account: (value, set) => {
                return (
                  <AccountSelectorConfig
                    account_type="git"
                    type="None"
                    provider={update.git_provider ?? config.git_provider}
                    selected={value}
                    onSelect={(git_account) => set({ git_account })}
                    disabled={disabled}
                    placeholder="None"
                  />
                );
              },
              repo: { placeholder: "Enter repo" },
              branch: { placeholder: "Enter branch" },
              commit: { placeholder: "Enter a specific commit hash. Optional." },
              resource_path: { placeholder: "./resources" },
              delete: { label: "Delete Unmatched Resources" },
            },
          },
          {
            label: "Github Webhooks",
            components: {
              ["refresh" as any]: () => (
                <ConfigItem label="Refresh Pending">
                  <CopyGithubWebhook path={`/sync/${id}/refresh`} />
                </ConfigItem>
              ),
              ["sync" as any]: () => (
                <ConfigItem label="Execute Sync">
                  <CopyGithubWebhook path={`/sync/${id}/sync`} />
                </ConfigItem>
              ),
              webhook_enabled: webhooks !== undefined && !webhooks.managed,
              ["managed" as any]: () => {
                const inv = useInvalidate();
                const { toast } = useToast();
                const { mutate: createWebhook, isPending: createPending } =
                  useWrite("CreateSyncWebhook", {
                    onSuccess: () => {
                      toast({ title: "Webhook Created" });
                      inv(["GetSyncWebhooksEnabled", { sync: id }]);
                    },
                  });
                const { mutate: deleteWebhook, isPending: deletePending } =
                  useWrite("DeleteSyncWebhook", {
                    onSuccess: () => {
                      toast({ title: "Webhook Deleted" });
                      inv(["GetSyncWebhooksEnabled", { sync: id }]);
                    },
                  });
                if (!webhooks || !webhooks.managed) return;
                return (
                  <ConfigItem label="Manage Webhook">
                    {webhooks.sync_enabled && (
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
                            SYNC EXECUTION
                          </div>
                        </div>
                        <ConfirmButton
                          title="Disable"
                          icon={<Ban className="w-4 h-4" />}
                          variant="destructive"
                          onClick={() =>
                            deleteWebhook({
                              sync: id,
                              action: Types.SyncWebhookAction.Sync,
                            })
                          }
                          loading={deletePending}
                          disabled={disabled || deletePending}
                        />
                      </div>
                    )}
                    {!webhooks.sync_enabled && webhooks.refresh_enabled && (
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
                            PENDING REFRESH
                          </div>
                        </div>
                        <ConfirmButton
                          title="Disable"
                          icon={<Ban className="w-4 h-4" />}
                          variant="destructive"
                          onClick={() =>
                            deleteWebhook({
                              sync: id,
                              action: Types.SyncWebhookAction.Refresh,
                            })
                          }
                          loading={deletePending}
                          disabled={disabled || deletePending}
                        />
                      </div>
                    )}
                    {!webhooks.sync_enabled && !webhooks.refresh_enabled && (
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
                          title="Enable Refresh"
                          icon={<CirclePlus className="w-4 h-4" />}
                          onClick={() =>
                            createWebhook({
                              sync: id,
                              action: Types.SyncWebhookAction.Refresh,
                            })
                          }
                          loading={createPending}
                          disabled={disabled || createPending}
                        />
                        <ConfirmButton
                          title="Enable Sync"
                          icon={<CirclePlus className="w-4 h-4" />}
                          onClick={() =>
                            createWebhook({
                              sync: id,
                              action: Types.SyncWebhookAction.Sync,
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
