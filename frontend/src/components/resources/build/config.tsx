import { Config } from "@components/config";
import {
  AccountSelectorConfig,
  AddExtraArgMenu,
  ConfigItem,
  ImageRegistryConfig,
  InputList,
  SystemCommand,
} from "@components/config/util";
import { useRead, useWrite } from "@lib/hooks";
import { env_to_text } from "@lib/utils";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { Textarea } from "@ui/textarea";
import { PlusCircle } from "lucide-react";
import { ReactNode, RefObject, createRef, useState } from "react";
import { CopyGithubWebhook, LabelsConfig, ResourceSelector } from "../common";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@ui/tabs";

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
  const global_disabled =
    useRead("GetCoreInfo", {}).data?.ui_write_disabled ?? false;
  // const docker_organizations = useRead("ListDockerOrganizations", {}).data;
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
                              version: { major: major + 1, minor, patch: 0 },
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
              repo: { placeholder: "Enter repo" },
              branch: { placeholder: "Enter branch" },
              commit: { placeholder: "Enter specific commit hash. Optional." },
              github_account:
                (update.builder_id ?? config.builder_id ? true : false) &&
                ((account, set) => (
                  <AccountSelectorConfig
                    id={update.builder_id ?? config.builder_id ?? undefined}
                    type="Builder"
                    account_type="github"
                    selected={account}
                    onSelect={(github_account) => set({ github_account })}
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
              // docker_account: (account, set) =>
              //   (update.builder_id ?? config.builder_id ? true : false) && (
              //     <AccountSelector
              //       id={update.builder_id ?? config.builder_id ?? undefined}
              //       type="Builder"
              //       account_type="docker"
              //       selected={account}
              //       onSelect={(docker_account) => set({ docker_account })}
              //       disabled={disabled}
              //       placeholder="None"
              //     />
              //   ),
              // docker_organization:
              //   docker_organizations === undefined ||
              //   docker_organizations.length === 0
              //     ? undefined
              //     : (value, set) => (
              //         <DockerOrganizations
              //           value={value}
              //           set={set}
              //           disabled={disabled}
              //         />
              //       ),
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
                <ConfigItem label="Build">
                  <CopyGithubWebhook path={`/build/${id}`} />
                </ConfigItem>
              ),
              webhook_enabled: true,
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
                  <BuildArgs args={args ?? ""} set={set} disabled={disabled} />
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

const BuildArgs = ({
  args,
  set,
  disabled,
}: {
  args: string;
  set: (input: Partial<Types.BuildConfig>) => void;
  disabled: boolean;
}) => {
  const ref = createRef<HTMLTextAreaElement>();
  const setArgs = (build_args: string) => set({ build_args });

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
  const { variables, secrets } = useRead("ListVariables", {}).data ?? {
    variables: [],
    secrets: [],
  };

  const _args = args || "";

  if (variables.length === 0 && secrets.length === 0) return;

  if (variables.length === 0) {
    // ONLY SECRETS
    return (
      <div className="flex flex-col gap-2 w-full">
        <h2 className="text-muted-foreground">Secrets</h2>
        <div className="flex gap-4 items-center flex-wrap w-full">
          {secrets.map((secret) => (
            <Button
              variant="secondary"
              key={secret}
              onClick={() =>
                setArgs(
                  _args.slice(0, argsRef.current?.selectionStart) +
                    `[[${secret}]]` +
                    _args.slice(argsRef.current?.selectionStart, undefined)
                )
              }
            >
              {secret}
            </Button>
          ))}
        </div>
      </div>
    );
  }

  if (secrets.length === 0) {
    // ONLY VARIABLES
    return (
      <div className="flex flex-col gap-2 w-full">
        <h2 className="text-muted-foreground">Variables</h2>
        <div className="flex gap-4 items-center flex-wrap w-full">
          {variables.map(({ name }) => (
            <Button
              variant="secondary"
              key={name}
              onClick={() =>
                setArgs(
                  _args.slice(0, argsRef.current?.selectionStart) +
                    `[[${name}]]` +
                    _args.slice(argsRef.current?.selectionStart, undefined)
                )
              }
            >
              {name}
            </Button>
          ))}
        </div>
      </div>
    );
  }

  return (
    <Tabs className="w-full" defaultValue="Variables">
      <TabsList>
        <TabsTrigger value="Variables">Variables</TabsTrigger>
        <TabsTrigger value="Secrets">Secrets</TabsTrigger>
      </TabsList>
      <TabsContent value="Variables">
        <div className="flex gap-4 items-center w-full flex-wrap pt-1">
          {variables.map(({ name }) => (
            <Button
              variant="secondary"
              key={name}
              onClick={() =>
                setArgs(
                  _args.slice(0, argsRef.current?.selectionStart) +
                    `[[${name}]]` +
                    _args.slice(argsRef.current?.selectionStart, undefined)
                )
              }
            >
              {name}
            </Button>
          ))}
        </div>
      </TabsContent>
      <TabsContent value="Secrets">
        <div className="flex gap-4 items-center w-full flex-wrap pt-1">
          {secrets.map((secret) => (
            <Button
              variant="secondary"
              key={secret}
              onClick={() =>
                setArgs(
                  _args.slice(0, argsRef.current?.selectionStart) +
                    `[[${secret}]]` +
                    _args.slice(argsRef.current?.selectionStart, undefined)
                )
              }
            >
              {secret}
            </Button>
          ))}
        </div>
      </TabsContent>
    </Tabs>
  );
};
