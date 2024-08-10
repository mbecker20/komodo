import { Config } from "@components/config";
import {
  AccountSelectorConfig,
  AddExtraArgMenu,
  ConfigItem,
  ImageRegistryConfig,
  InputList,
  ProviderSelectorConfig,
  SecretsForEnvironment,
  SystemCommand,
} from "@components/config/util";
import { useInvalidate, useRead, useWrite } from "@lib/hooks";
import { env_to_text } from "@lib/utils";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { Textarea } from "@ui/textarea";
import { Ban, CirclePlus, PlusCircle } from "lucide-react";
import { ReactNode, createRef, useState } from "react";
import {
  BuilderSelector,
  CopyGithubWebhook,
  LabelsConfig,
} from "../common";
import { useToast } from "@ui/use-toast";
import { text_color_class_by_intention } from "@lib/color";
import { ConfirmButton } from "@components/util";

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
              version: (version, set) => {
                const { major, minor, patch } = version ?? {
                  major: 0,
                  minor: 0,
                  patch: 0,
                };
                return (
                  <ConfigItem label="Version">
                    <div className="flex gap-4 items-center">
                      <div className="text-xl">
                        v{major}.{minor}.{patch}
                      </div>
                      {!disabled && (
                        <Button
                          variant="secondary"
                          onClick={() =>
                            set({
                              version: { major: major + 1, minor: 0, patch: 0 },
                            })
                          }
                        >
                          + Major
                        </Button>
                      )}
                      {!disabled && (
                        <Button
                          variant="secondary"
                          onClick={() =>
                            set({
                              version: { major, minor: minor + 1, patch: 0 },
                            })
                          }
                        >
                          + Minor
                        </Button>
                      )}
                    </div>
                  </ConfigItem>
                );
              },
              builder_id: (id, set) => (
                <BuilderSelector selected={id} set={set} disabled={disabled} />
              ),
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
              repo: { placeholder: "Enter repo" },
              branch: { placeholder: "Enter branch" },
              commit: {
                placeholder: "Enter a specific commit hash. Optional.",
              },
            },
          },
          {
            label: "Docker",
            components: {
              image_registry: (registry, set) => (
                <ImageRegistryConfig
                  registry={registry}
                  setRegistry={(image_registry) => set({ image_registry })}
                  resource_id={update.builder_id ?? config.builder_id}
                  disabled={disabled}
                />
              ),
              image_name: {
                description: "Optional. Push the image under a different name",
                placeholder: "Custom image name",
              },
              image_tag: {
                description:
                  "Optional. Postfix the image version with a custom tag.",
                placeholder: "Custom image tag",
              },
              build_path: true,
              dockerfile_path: true,
            },
          },
          {
            label: "Extra Args",
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
            label: "Labels",
            contentHidden: (update.labels ?? config.labels)?.length === 0,
            actions: !disabled && (
              <Button
                variant="secondary"
                onClick={() =>
                  set((update) => {
                    return {
                      ...update,
                      labels: [
                        ...(update.labels ?? config.labels ?? []),
                        { variable: "", value: "" },
                      ] as Types.EnvironmentVar[],
                    };
                  })
                }
                className="flex items-center gap-2 w-[200px]"
              >
                <PlusCircle className="w-4 h-4" />
                Add Label
              </Button>
            ),
            components: {
              labels: (l, set) => (
                <LabelsConfig
                  labels={(l as Types.EnvironmentVar[]) ?? []}
                  set={set}
                  disabled={disabled}
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
            label: "Github Webhook",
            description:
              "Configure your repo provider to send webhooks to Monitor",
            components: {
              ["build" as any]: () => (
                <ConfigItem label="Webhook Url">
                  <CopyGithubWebhook path={`/build/${id}`} />
                </ConfigItem>
              ),
              webhook_enabled: webhook !== undefined && !webhook.managed,
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
        ],
        "Build Args": [
          {
            label: "Build Args",
            components: {
              build_args: (vars, set) => {
                const args =
                  typeof vars === "object" ? env_to_text(vars) : vars;
                return (
                  <Args
                    type="build"
                    args={args ?? ""}
                    set={set}
                    disabled={disabled}
                  />
                );
              },
              skip_secret_interp: true,
            },
          },
        ],
        "Secret Args": [
          {
            label: "Secret Args",
            components: {
              secret_args: (vars, set) => {
                const args =
                  typeof vars === "object" ? env_to_text(vars) : vars;
                return (
                  <Args
                    type="secret"
                    args={args ?? ""}
                    set={set}
                    disabled={disabled}
                  />
                );
              },
              skip_secret_interp: true,
            },
          },
        ],
      }}
    />
  );
};

const Args = ({
  type,
  args,
  set,
  disabled,
}: {
  type: "build" | "secret";
  args: string;
  set: (input: Partial<Types.BuildConfig>) => void;
  disabled: boolean;
}) => {
  const ref = createRef<HTMLTextAreaElement>();
  const setArgs = (args: string) => set({ [`${type}_args`]: args });

  return (
    <ConfigItem className="flex-col gap-4 items-start">
      {!disabled && (
        <SecretsForEnvironment env={args} setEnv={setArgs} envRef={ref} />
      )}
      <Textarea
        ref={ref}
        className="min-h-[400px]"
        placeholder="VARIABLE=value"
        value={args}
        onChange={(e) => setArgs(e.target.value)}
        disabled={disabled}
      />
    </ConfigItem>
  );
};
