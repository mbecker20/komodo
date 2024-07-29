import { Config } from "@components/config";
import {
  AccountSelectorConfig,
  AddExtraArgMenu,
  ConfigItem,
  ImageRegistryConfig,
  InputList,
  ProviderSelectorConfig,
  SecretSelector,
  SystemCommand,
} from "@components/config/util";
import { useInvalidate, useRead, useWrite } from "@lib/hooks";
import { env_to_text } from "@lib/utils";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { Textarea } from "@ui/textarea";
import { Ban, CirclePlus, PlusCircle } from "lucide-react";
import { ReactNode, RefObject, createRef, useState } from "react";
import { CopyGithubWebhook, LabelsConfig, ResourceSelector } from "../common";
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
                <ConfigItem label="Builder">
                  <ResourceSelector
                    type="Builder"
                    selected={id}
                    onSelect={(builder_id) => set({ builder_id })}
                    disabled={disabled}
                    align="end"
                  />
                </ConfigItem>
              ),
            },
          },
          {
            label: "Git",
            components: {
              git_provider: (provider, set) => (
                <ProviderSelectorConfig
                  account_type="git"
                  selected={provider}
                  disabled={disabled}
                  onSelect={(git_provider) => set({ git_provider })}
                />
              ),
              repo: { placeholder: "Enter repo" },
              branch: { placeholder: "Enter branch" },
              commit: { placeholder: "Enter specific commit hash. Optional." },
              git_account:
                (update.builder_id ?? config.builder_id ? true : false) &&
                ((account, set) => (
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
                )),
            },
          },
          {
            label: "Docker",
            components: {
              image_registry: (registry, set) => {
                const builder_id = update.builder_id ?? config.builder_id;
                if (!builder_id) return null;
                return (
                  <ImageRegistryConfig
                    registry={registry}
                    setRegistry={(image_registry) => set({ image_registry })}
                    type="Build"
                    resource_id={builder_id}
                    disabled={disabled}
                  />
                );
              },
              build_path: true,
              dockerfile_path: true,
              use_buildx: true,
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
      {!disabled && <Secrets args={args} setArgs={setArgs} argsRef={ref} />}
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

const Secrets = ({
  args,
  setArgs,
  argsRef,
}: {
  args?: string;
  setArgs: (args: string) => void;
  argsRef: RefObject<HTMLTextAreaElement>;
}) => {
  const variables = useRead("ListVariables", {}).data ?? [];
  const secrets = useRead("ListSecrets", {}).data ?? [];

  const _args = args || "";

  if (variables.length === 0 && secrets.length === 0) return;

  return (
    <div className="flex items-center gap-2">
      {variables.length > 0 && (
        <SecretSelector
          type="Variable"
          keys={variables.map((v) => v.name)}
          onSelect={(variable) =>
            setArgs(
              _args.slice(0, argsRef.current?.selectionStart) +
                `[[${variable}]]` +
                _args.slice(argsRef.current?.selectionStart, undefined)
            )
          }
          disabled={false}
        />
      )}
      {secrets.length > 0 && (
        <SecretSelector
          type="Secret"
          keys={secrets}
          onSelect={(secret) =>
            setArgs(
              _args.slice(0, argsRef.current?.selectionStart) +
                `[[${secret}]]` +
                _args.slice(argsRef.current?.selectionStart, undefined)
            )
          }
          disabled={false}
        />
      )}
    </div>
  );
};
