import { Config } from "@components/config";
import {
  AccountSelector,
  ConfigItem,
  SystemCommand,
} from "@components/config/util";
import { useRead, useWrite } from "@lib/hooks";
import { env_to_text, text_to_env } from "@lib/utils";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { Input } from "@ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";
import { Textarea } from "@ui/textarea";
import { MinusCircle, PlusCircle } from "lucide-react";
import { useEffect, useState } from "react";
import { CopyGithubWebhook, LabelsConfig, ResourceSelector } from "../common";

export const BuildConfig = ({ id }: { id: string }) => {
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
      disabled={disabled}
      config={config}
      update={update}
      set={set}
      onSave={async () => {
        await mutateAsync({ id, config: update });
      }}
      components={{
        general: {
          general: {
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
          git: {
            repo: true,
            branch: true,
            commit: true,
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
          docker: {
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
            labels: (l, set) => (
              <LabelsConfig labels={l ?? []} set={set} disabled={disabled} />
            ),
            extra_args: (value, set) => (
              <ExtraArgs args={value ?? []} set={set} disabled={disabled} />
            ),
          },
          pre_build: {
            pre_build: (value, set) => (
              <SystemCommand
                label="Pre Build"
                value={value}
                set={(value) => set({ pre_build: value })}
                disabled={disabled}
              />
            ),
          },
          github_webhook: {
            ["build" as any]: () => (
              <ConfigItem label="Build">
                <CopyGithubWebhook path={`/build/${id}`} />
              </ConfigItem>
            ),
            webhook_enabled: true,
          },
        },
        "Build Args": {
          "Build Args": {
            build_args: (vars, set) => (
              <BuildArgs vars={vars ?? []} set={set} disabled={disabled} />
            ),
            skip_secret_interp: true,
          },
        },
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

const ExtraArgs = ({
  args,
  set,
  disabled,
}: {
  args: string[];
  set: (update: Partial<Types.BuildConfig>) => void;
  disabled: boolean;
}) => {
  return (
    <ConfigItem
      label="Extra Args"
      className={args.length > 0 ? "items-start" : undefined}
    >
      <div className="flex flex-col gap-4 w-full max-w-[400px]">
        {args.map((arg, i) => (
          <div className="w-full flex gap-4" key={i}>
            <Input
              value={arg}
              placeholder="--extra-arg=value"
              onChange={(e) => {
                args[i] = e.target.value;
                set({ extra_args: [...args] });
              }}
              disabled={disabled}
            />
            {!disabled && (
              <Button
                variant="secondary"
                onClick={() =>
                  set({ extra_args: [...args.filter((_, idx) => idx !== i)] })
                }
              >
                <MinusCircle className="w-4 h-4" />
              </Button>
            )}
          </div>
        ))}

        <Button
          variant="secondary"
          className="flex items-center gap-2 w-[200px] place-self-end"
          onClick={() => set({ extra_args: [...args, ""] })}
        >
          <PlusCircle className="w-4 h-4" /> Add Extra Arg
        </Button>
      </div>
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
