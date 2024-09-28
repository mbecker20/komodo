import { Config } from "@components/config";
import {
  AccountSelectorConfig,
  ConfigItem,
  InputList,
  ProviderSelectorConfig,
} from "@components/config/util";
import { useInvalidate, useLocalStorage, useRead, useWrite } from "@lib/hooks";
import { Types } from "@komodo/client";
import { ReactNode, useState } from "react";
import { CopyGithubWebhook } from "../common";
import { useToast } from "@ui/use-toast";
import { text_color_class_by_intention } from "@lib/color";
import { ConfirmButton, ShowHideButton } from "@components/util";
import { Ban, CirclePlus, PlusCircle } from "lucide-react";
import { Button } from "@ui/button";
import { MonacoEditor } from "@components/monaco";

export const ResourceSyncConfig = ({
  id,
  titleOther,
}: {
  id: string;
  titleOther: ReactNode;
}) => {
  const [showFile, setShowFile] = useLocalStorage(`sync-${id}-show-file`, true);
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
  const files_on_host = update.files_on_host ?? config.files_on_host;
  const file_contents =
    update.file_contents ?? config.file_contents ? true : false;
  const ui_defined = !files_on_host && file_contents;
  const repo_selected = update.repo ?? config.repo ? true : false;
  const managed = update.managed ?? config.managed;

  const show_git = !managed && !files_on_host && !ui_defined;

  return (
    <Config
      resource_id={id}
      resource_type="ResourceSync"
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
            label: "Resource File",
            hidden: files_on_host || repo_selected,
            description:
              "Manage the resource file contents here, or use a git repo / the files on host option.",
            actions: <ShowHideButton show={showFile} setShow={setShowFile} />,
            contentHidden: !showFile,
            components: {
              file_contents: (file_contents, set) => {
                return (
                  <MonacoEditor
                    value={
                      file_contents ||
                      "# Initialize the sync to import your current resources.\n"
                    }
                    onValueChange={(file_contents) => set({ file_contents })}
                    language="toml"
                  />
                );
              },
            },
          },
          {
            label: "General",
            components: {
              files_on_host: {
                label: "Files on Server",
                description:
                  "Manage the sync files on server yourself. Just configure the path to your folder / file.",
              },
              delete: !managed && {
                label: "Delete Unmatched Resources",
                description:
                  "Executions will delete any resources not found in the resource files. Only use this when using one sync for everything.",
              },
              resource_path: (files_on_host ||
                (!ui_defined && repo_selected)) && {
                placeholder: "./resources",
                description:
                  "Provide the path to resource file / folder, in the container filesystem or from the root of the repo",
              },
              managed: (managed ||
                files_on_host ||
                ui_defined ||
                !repo_selected) && {
                label: "Managed",
                description:
                  "Enabled managed mode / the 'Commit' button. Commit is the 'reverse' of Execute, and will update the sync file with your configs updated in the UI.",
              },
            },
          },
          {
            label: "Match Tags",
            contentHidden:
              (update.match_tags ?? config.match_tags)?.length === 0,
            actions: !disabled && (
              <Button
                variant="secondary"
                onClick={() =>
                  set((update) => ({
                    ...update,
                    match_tags: [
                      ...(update.match_tags ?? config.match_tags ?? []),
                      "",
                    ],
                  }))
                }
                className="flex items-center gap-2 w-[200px]"
              >
                <PlusCircle className="w-4 h-4" />
                Add Tag
              </Button>
            ),
            components: {
              match_tags: (values, set) => (
                <InputList
                  field="match_tags"
                  values={values ?? []}
                  set={set}
                  disabled={disabled}
                  placeholder="Tag"
                />
              ),
            } as any,
          },
          {
            label: "Git Repo",
            hidden: !show_git,
            components: {
              git_provider: (provider: string | undefined, set) => {
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
              git_account: (value: string | undefined, set) => {
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
            label: "Git Webhooks",
            hidden: !show_git,
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
              ["refresh" as any]: () => (
                <ConfigItem
                  label="Refresh Pending"
                  description="Trigger an update of the pending sync cache, to display the changes in the UI on push."
                >
                  <CopyGithubWebhook path={`/sync/${id}/refresh`} />
                </ConfigItem>
              ),
              ["sync" as any]: () => (
                <ConfigItem
                  label="Execute Sync"
                  description="Trigger an execution of the sync on push."
                >
                  <CopyGithubWebhook path={`/sync/${id}/sync`} />
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
