import { Config } from "@components/config";
import {
  AccountSelectorConfig,
  AddExtraArgMenu,
  ConfigInput,
  ConfigItem,
  ImageRegistryConfig,
  InputList,
  ProviderSelectorConfig,
  SystemCommand,
} from "@components/config/util";
import { useInvalidate, useRead, useWrite } from "@lib/hooks";
import { Types } from "@komodo/client";
import { Button } from "@ui/button";
import { Ban, CirclePlus, PlusCircle } from "lucide-react";
import { ReactNode, useState } from "react";
import { BuilderSelector, CopyGithubWebhook, ResourceSelector } from "../common";
import { useToast } from "@ui/use-toast";
import { text_color_class_by_intention } from "@lib/color";
import { ConfirmButton } from "@components/util";
import { Link } from "react-router-dom";
import { SecretsSearch } from "@components/config/env_vars";
import { MonacoEditor } from "@components/monaco";

export const BuildConfig = ({
  id,
  titleOther,
}: {
  id: string;
  titleOther: ReactNode;
}) => {
  const perms = useRead("GetPermissionLevel", {
    target: { type: "Build", id },
  }).data;
  const config = useRead("GetBuild", { build: id }).data?.config;
  const webhook = useRead("GetBuildWebhookEnabled", { build: id }).data;
  const global_disabled =
    useRead("GetCoreInfo", {}).data?.ui_write_disabled ?? false;
  const [update, set] = useState<Partial<Types.BuildConfig>>({});
  const { mutateAsync } = useWrite("UpdateBuild");

  if (!config) return null;

  const disabled = global_disabled || perms !== Types.PermissionLevel.Write;

  return (
    <Config
      resource_id={id}
      resource_type="Build"
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
            label: "Builder",
            contentHidden: true,
            actions: (
              <ResourceSelector
                type="Builder"
                selected={update.builder_id ?? config.builder_id}
                onSelect={(builder_id) => set({ builder_id })}
                disabled={disabled}
                align="end"
              />
            ),
            components: {},
          },
          {
            label: "Version",
            labelHidden: true,
            components: {
              version: (_version, set) => {
                const version =
                  typeof _version === "object"
                    ? `${_version.major}.${_version.minor}.${_version.patch}`
                    : _version;
                return (
                  <ConfigInput
                    className="text-lg w-[200px]"
                    label="Version"
                    placeholder="0.0.0"
                    value={version}
                    onChange={(version) => set({ version: version as any })}
                    disabled={disabled}
                    boldLabel
                  />
                );
              },
              auto_increment_version: true,
            },
          },
          {
            label: "Git",
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
              git_account: (account, set) => (
                <AccountSelectorConfig
                  id={update.builder_id ?? config.builder_id ?? undefined}
                  type="Builder"
                  account_type="git"
                  provider={update.git_provider ?? config.git_provider}
                  selected={account}
                  onSelect={(git_account) => set({ git_account })}
                  disabled={disabled}
                  placeholder="None"
                />
              ),
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
            label: "Image",
            components: {
              image_registry: (registry, set) => (
                <ImageRegistryConfig
                  registry={registry}
                  setRegistry={(image_registry) => set({ image_registry })}
                  resource_id={update.builder_id ?? config.builder_id}
                  disabled={disabled}
                />
              ),
              build_path: {
                placeholder: ".",
                description:
                  "The cwd to run 'docker build', relative to the root of the repo.",
              },
              dockerfile_path: {
                placeholder: "Dockerfile",
                description:
                  "The path to the dockerfile, relative to the build path.",
              },
            },
          },
          {
            label: "Custom Name / Tag",
            components: {
              image_name: {
                description: "Optional. Push the image under a different name",
                placeholder: "Custom image name",
              },
              image_tag: {
                description:
                  "Optional. Postfix the image version with a custom tag.",
                placeholder: "Custom image tag",
              },
            },
          },
          {
            label: "Extra Args",
            description: (
              <div className="flex flex-row flex-wrap">
                <div>Pass extra arguments to 'docker build'.</div>
                <Link
                  to="https://docs.docker.com/reference/cli/docker/buildx/build/"
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
                type="Build"
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
            label: "Build Args",
            description:
              "Pass build args to 'docker build'. These can be used in the Dockerfile via ARG, and are visible in the final image.",
            labelExtra: !disabled && <SecretsSearch />,
            components: {
              build_args: (env, set) => (
                <MonacoEditor
                  value={env || "  # VARIABLE: value"}
                  onValueChange={(build_args) => set({ build_args })}
                  language="yaml"
                  readOnly={disabled}
                />
              ),
            },
          },
          {
            label: "Secret Args",
            description: (
              <div className="flex flex-row flex-wrap">
                <div>
                  Pass secrets to 'docker build'. These values remain hidden in
                  the final image by using docker secret mounts.
                </div>
                <Link
                  to="https://docs.rs/komodo_client/latest/komodo_client/entities/build/struct.BuildConfig.html#structfield.secret_args"
                  target="_blank"
                >
                  <Button variant="link" className="p-0">
                    See the docs.
                  </Button>
                </Link>
              </div>
            ),
            labelExtra: !disabled && <SecretsSearch />,
            components: {
              secret_args: (env, set) => (
                <MonacoEditor
                  value={env || "  # VARIABLE: value"}
                  onValueChange={(secret_args) => set({ secret_args })}
                  language="yaml"
                  readOnly={disabled}
                />
              ),
            },
          },
          {
            label: "Labels",
            description: "Attach --labels to image.",
            components: {
              labels: (labels, set) => (
                <MonacoEditor
                  value={labels || "  # your.docker.label: value"}
                  language="yaml"
                  onValueChange={(labels) => set({ labels })}
                  readOnly={disabled}
                />
              ),
            },
          },
          {
            label: "Pre Build",
            description:
              "Execute a shell command before running docker build. The given Cwd is relative to repo root.",
            components: {
              pre_build: (value, set) => (
                <SystemCommand
                  value={value}
                  set={(value) => set({ pre_build: value })}
                  disabled={disabled}
                />
              ),
            },
          },
          {
            label: "Git Webhook",
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
              ["build" as any]: () => (
                <ConfigItem label="Webhook Url">
                  <CopyGithubWebhook path={`/build/${id}`} />
                </ConfigItem>
              ),
              webhook_enabled: webhook !== undefined && !webhook.managed,
              webhook_secret: {
                description:
                  "Provide a custom webhook secret for this resource, or use the global default.",
                placeholder: "Input custom secret",
              },
              ["managed" as any]: () => {
                const inv = useInvalidate();
                const { toast } = useToast();
                const { mutate: createWebhook, isPending: createPending } =
                  useWrite("CreateBuildWebhook", {
                    onSuccess: () => {
                      toast({ title: "Webhook Created" });
                      inv(["GetBuildWebhookEnabled", { build: id }]);
                    },
                  });
                const { mutate: deleteWebhook, isPending: deletePending } =
                  useWrite("DeleteBuildWebhook", {
                    onSuccess: () => {
                      toast({ title: "Webhook Deleted" });
                      inv(["GetBuildWebhookEnabled", { build: id }]);
                    },
                  });
                if (!webhook || !webhook.managed) return;
                return (
                  <ConfigItem label="Manage Webhook">
                    {webhook.enabled && (
                      <div className="flex items-center gap-4 flex-wrap">
                        <div className="flex items-center gap-2">
                          Incoming webhook is{" "}
                          <div
                            className={text_color_class_by_intention("Good")}
                          >
                            ENABLED
                          </div>
                        </div>
                        <ConfirmButton
                          title="Disable"
                          icon={<Ban className="w-4 h-4" />}
                          variant="destructive"
                          onClick={() => deleteWebhook({ build: id })}
                          loading={deletePending}
                          disabled={disabled || deletePending}
                        />
                      </div>
                    )}
                    {!webhook.enabled && (
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
                          title="Enable Build"
                          icon={<CirclePlus className="w-4 h-4" />}
                          onClick={() => createWebhook({ build: id })}
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
