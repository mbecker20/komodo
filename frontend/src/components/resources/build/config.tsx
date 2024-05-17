import { Config } from "@components/config";
import {
  AccountSelector,
  AddExtraArgMenu,
  ConfigItem,
  InputList,
  SystemCommand,
} from "@components/config/util";
import { useRead, useWrite } from "@lib/hooks";
import { env_to_text, text_to_env } from "@lib/utils";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";
import { Textarea } from "@ui/textarea";
import { PlusCircle } from "lucide-react";
import { ReactNode, useEffect, useState } from "react";
import { CopyGithubWebhook, LabelsConfig, ResourceSelector } from "../common";

export const BuildConfig = ({ id, titleOther }: { id: string; titleOther: ReactNode }) => {
  const perms = useRead("GetPermissionLevel", {
    target: { type: "Build", id },
  }).data;
  const config = useRead("GetBuild", { build: id }).data?.config;
  const docker_organizations = useRead("ListDockerOrganizations", {}).data;
  const [update, set] = useState<Partial<Types.BuildConfig>>({});
  const { mutateAsync } = useWrite("UpdateBuild");

  if (!config) return null;

  const disabled = perms !== Types.PermissionLevel.Write;

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
              github_account: (account, set) => (
                <AccountSelector
                  id={update.builder_id ?? config.builder_id ?? undefined}
                  type="Builder"
                  account_type="github"
                  selected={account}
                  onSelect={(github_account) => set({ github_account })}
                  disabled={disabled}
                  placeholder="None"
                />
              ),
            },
          },
          {
            label: "Docker",
            components: {
              build_path: true,
              dockerfile_path: true,
              docker_account: (account, set) => (
                <AccountSelector
                  id={update.builder_id ?? config.builder_id ?? undefined}
                  type="Builder"
                  account_type="docker"
                  selected={account}
                  onSelect={(docker_account) => set({ docker_account })}
                  disabled={disabled}
                  placeholder="None"
                />
              ),
              docker_organization:
                docker_organizations === undefined ||
                docker_organizations.length === 0
                  ? undefined
                  : (value, set) => (
                      <DockerOrganizations
                        value={value}
                        set={set}
                        disabled={disabled}
                      />
                    ),
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
                  set({
                    extra_args: [
                      ...(update.extra_args ?? config.extra_args ?? []),
                      suggestion,
                    ],
                  })
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
                  set({
                    ...update,
                    labels: [
                      ...(update.labels ?? config.labels ?? []),
                      { variable: "", value: "" },
                    ],
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
                <LabelsConfig labels={l ?? []} set={set} disabled={disabled} />
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
              build_args: (vars, set) => (
                <BuildArgs vars={vars ?? []} set={set} disabled={disabled} />
              ),
              skip_secret_interp: true,
            },
          },
        ],
      }}
    />
  );
};

const BuildArgs = ({
  vars,
  set,
  disabled,
}: {
  vars: Types.EnvironmentVar[];
  set: (input: Partial<Types.BuildConfig>) => void;
  disabled: boolean;
}) => {
  const [args, setArgs] = useState(env_to_text(vars));
  useEffect(() => {
    !!args && set({ build_args: text_to_env(args) });
  }, [args, set]);

  return (
    <ConfigItem className="flex-col gap-4 items-start">
      <Textarea
        className="min-h-[300px]"
        placeholder="VARIABLE=value"
        value={args}
        onChange={(e) => setArgs(e.target.value)}
        disabled={disabled}
      />
    </ConfigItem>
  );
};

const DockerOrganizations = ({
  value,
  set,
  disabled,
}: {
  value?: string;
  set: (input: Partial<Types.BuildConfig>) => void;
  disabled: boolean;
}) => {
  const docker_organizations = useRead("ListDockerOrganizations", {}).data;
  return (
    <ConfigItem label="Docker Organization">
      <Select
        value={value}
        onValueChange={(value) => set({ docker_organization: value })}
        disabled={disabled}
      >
        <SelectTrigger
          className="w-full lg:w-[300px] max-w-[50%]"
          disabled={disabled}
        >
          <SelectValue placeholder="Select Organization" />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value={""}>None</SelectItem>
          {docker_organizations?.map((org) => (
            <SelectItem key={org} value={org}>
              {org}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </ConfigItem>
  );
};
