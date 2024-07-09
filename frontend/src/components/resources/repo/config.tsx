import { Config } from "@components/config";
import {
  AccountSelectorConfig,
  ConfigItem,
  SystemCommand,
} from "@components/config/util";
import { useInvalidate, useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { useState } from "react";
import { CopyGithubWebhook, ServerSelector } from "../common";
import { useToast } from "@ui/use-toast";
import { text_color_class_by_intention } from "@lib/color";
import { ConfirmButton } from "@components/util";
import { Ban, CirclePlus } from "lucide-react";

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
              repo: { placeholder: "Enter repo" },
              branch: { placeholder: "Enter branch" },
              commit: { placeholder: "Enter specific commit hash. Optional." },
              path: {
                placeholder: "Enter a specific clone path. Optional.",
              },
              github_account: (value, set) => {
                const server_id = update.server_id || config.server_id;
                return (
                  <AccountSelectorConfig
                    id={server_id}
                    account_type="github"
                    type={server_id ? "Server" : "None"}
                    selected={value}
                    onSelect={(github_account) => set({ github_account })}
                    disabled={disabled}
                    placeholder="None"
                  />
                );
              },
            },
          },
          {
            label: "On Clone",
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
            label: "Github Webhooks",
            components: {
              ["clone" as any]: () => (
                <ConfigItem label="Clone">
                  <CopyGithubWebhook path={`/repo/${id}/clone`} />
                </ConfigItem>
              ),
              ["pull" as any]: () => (
                <ConfigItem label="Pull">
                  <CopyGithubWebhook path={`/repo/${id}/pull`} />
                </ConfigItem>
              ),
              webhook_enabled: webhooks !== undefined && !webhooks.managed,
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
                    {!webhooks.clone_enabled && !webhooks.pull_enabled && (
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
