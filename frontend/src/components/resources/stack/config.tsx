import { Config } from "@components/config";
import {
  AccountSelectorConfig,
  AddExtraArgMenu,
  ConfigItem,
  InputList,
  ProviderSelectorConfig,
  SecretsForEnvironment,
} from "@components/config/util";
import { useInvalidate, useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { createRef, ReactNode, useState } from "react";
import { CopyGithubWebhook, ServerSelector } from "../common";
import { useToast } from "@ui/use-toast";
import { text_color_class_by_intention } from "@lib/color";
import { ConfirmButton } from "@components/util";
import {
  Ban,
  ChevronDown,
  ChevronUp,
  CirclePlus,
  PlusCircle,
} from "lucide-react";
import { env_to_text } from "@lib/utils";
import { Textarea } from "@ui/textarea";
import { Button } from "@ui/button";

export const StackConfig = ({
  id,
  titleOther,
}: {
  id: string;
  titleOther: ReactNode;
}) => {
  const perms = useRead("GetPermissionLevel", {
    target: { type: "Stack", id },
  }).data;
  const config = useRead("GetStack", { stack: id }).data?.config;
  const webhooks = useRead("GetStackWebhooksEnabled", { stack: id }).data;
  const global_disabled =
    useRead("GetCoreInfo", {}).data?.ui_write_disabled ?? false;
  const [update, set] = useState<Partial<Types.StackConfig>>({});
  const { mutateAsync } = useWrite("UpdateStack");
  const [fileContentsOpen, setFileContentsOpen] = useState(false);
  const fileContentsRef = createRef<HTMLTextAreaElement>();

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
            label: "Project Name",
            labelHidden: true,
            components: {
              project_name: {
                boldLabel: true,
                placeholder: "Compose project name",
                description:
                  "Optionally override the compose project name. Can import stacks by matching the existing project name on your host.",
              },
            },
          },
          {
            label: "Compose File",
            description:
              "Paste the file contents here, or configure a git repo.",
            actions: (
              <Button
                size="sm"
                variant="outline"
                className="gap-4"
                onClick={() => setFileContentsOpen(!fileContentsOpen)}
              >
                {fileContentsOpen ? "Hide" : "Show"}
                {fileContentsOpen ? (
                  <ChevronUp className="w-4" />
                ) : (
                  <ChevronDown className="w-4" />
                )}
              </Button>
            ),
            contentHidden: !fileContentsOpen,
            components: {
              file_contents: (file_contents, set) => {
                return (
                  <Textarea
                    ref={fileContentsRef}
                    value={file_contents}
                    onChange={(e) => set({ file_contents: e.target.value })}
                    className="min-h-[300px] h-fit"
                    placeholder="Paste compose file contents"
                    spellCheck={false}
                    onKeyDown={(e) => {
                      if (e.key === "Tab") {
                        e.preventDefault();
                        if (!fileContentsRef.current) return;

                        const start = fileContentsRef.current.selectionStart;
                        const end = fileContentsRef.current.selectionEnd;

                        const SPACE_COUNT = 4;

                        // set textarea value to: text before caret + tab + text after caret
                        fileContentsRef.current.value =
                          fileContentsRef.current.value.substring(0, start) +
                          // Use four spaces for indent
                          " ".repeat(SPACE_COUNT) +
                          fileContentsRef.current.value.substring(end);

                        // put caret at right position again
                        fileContentsRef.current.selectionStart =
                          fileContentsRef.current.selectionEnd =
                            start + SPACE_COUNT;
                      }
                    }}
                  />
                );
              },
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
              ((update.extra_args ?? config.extra_args)?.length ?? 0) === 0,
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
        ],
        "Git Repo": [
          {
            label: "Git",
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
                placeholder: "Eg. './'",
                description:
                  "Set the cwd when running compose up command. Relative to the repo root.",
                boldLabel: true,
              },
            },
          },
          {
            label: "File Paths",
            description:
              "Add files to include using 'docker compose -f'. If empty, uses 'compose.yaml'.",
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
              "Configure your repo provider to send webhooks to Monitor",
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
        environment: [
          {
            label: "Environment",
            description: "Pass these variables to the compose command",
            components: {
              environment: (env, set) => {
                const _env = typeof env === "object" ? env_to_text(env) : env;
                return (
                  <Environment env={_env ?? ""} set={set} disabled={disabled} />
                );
              },
              env_file_path: {
                description:
                  "The path to write the file to, relative to the root of the repo.",
                placeholder: ".env",
              },
              skip_secret_interp: true,
            },
          },
        ],
      }}
    />
  );
};

const Environment = ({
  env,
  set,
  disabled,
}: {
  env: string;
  set: (input: Partial<Types.StackConfig>) => void;
  disabled: boolean;
}) => {
  const ref = createRef<HTMLTextAreaElement>();
  const setEnv = (environment: string) => set({ environment });
  return (
    <ConfigItem className="flex-col gap-4 items-start">
      {!disabled && (
        <SecretsForEnvironment env={env} setEnv={setEnv} envRef={ref} />
      )}
      <Textarea
        ref={ref}
        className="min-h-[400px]"
        placeholder="VARIABLE=value"
        value={env}
        onChange={(e) => setEnv(e.target.value)}
        disabled={disabled}
        spellCheck={false}
      />
    </ConfigItem>
  );
};
