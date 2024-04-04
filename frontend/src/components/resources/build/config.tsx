import { Config } from "@components/config";
import {
  AccountSelector,
  ConfigItem,
  ResourceSelector,
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

export const BuildConfig = ({ id }: { id: string }) => {
  const config = useRead("GetBuild", { build: id }).data?.config;
  const docker_organizations = useRead("ListDockerOrganizations", {}).data;
  const [update, set] = useState<Partial<Types.BuildConfig>>({});
  const { mutate } = useWrite("UpdateBuild");

  if (!config) return null;

  return (
    <Config
      config={config}
      update={update}
      set={set}
      onSave={() => mutate({ id, config: update })}
      components={{
        general: {
          general: {
            builder_id: (id, set) => (
              <div className="flex justify-between items-center border-b pb-4 min-h-[60px]">
                <div>Builder</div>
                <ResourceSelector
                  type="Builder"
                  selected={id}
                  onSelect={(builder_id) => set({ builder_id })}
                />
              </div>
            ),
          },
          git: {
            repo: true,
            branch: true,
            github_account: (account, set) => (
              <AccountSelector
                id={update.builder_id ?? config.builder_id ?? undefined}
                type="Builder"
                account_type="github"
                selected={account}
                onSelect={(github_account) => set({ github_account })}
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
              />
            ),
            docker_organization:
              docker_organizations === undefined ||
              docker_organizations.length === 0
                ? undefined
                : (value, set) => (
                    <DockerOrganizations value={value} set={set} />
                  ),
            use_buildx: true,
            // docker_organization,
            extra_args: (value, set) => (
              <ExtraArgs args={value ?? []} set={set} />
            ),
          },
          pre_build: {
            pre_build: (value, set) => (
              <SystemCommand
                label="Pre Build"
                value={value}
                set={(value) => set({ pre_build: value })}
              />
            ),
          },
        },
        "Build Args": {
          "Build Args": {
            build_args: (vars, set) => (
              <BuildArgs vars={vars ?? []} set={set} />
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
}: {
  vars: Types.EnvironmentVar[];
  set: (input: Partial<Types.BuildConfig>) => void;
}) => {
  const [args, setArgs] = useState(env_to_text(vars));
  useEffect(() => {
    !!args && set({ build_args: text_to_env(args) });
  }, [args, set]);

  return (
    <ConfigItem label="Build Args" className="flex-col gap-4 items-start">
      <Textarea
        className="min-h-[300px]"
        placeholder="VARIABLE=value"
        value={args}
        onChange={(e) => setArgs(e.target.value)}
      />
    </ConfigItem>
  );
};

const ExtraArgs = ({
  args,
  set,
}: {
  args: string[];
  set: (update: Partial<Types.BuildConfig>) => void;
}) => {
  return (
    <ConfigItem label="Extra Args" className="items-start">
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
            />
            <Button
              variant="outline"
              onClick={() =>
                set({ extra_args: [...args.filter((_, idx) => idx !== i)] })
              }
            >
              <MinusCircle className="w-4 h-4" />
            </Button>
          </div>
        ))}

        <Button
          variant="outline"
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
}: {
  value?: string;
  set: (input: Partial<Types.BuildConfig>) => void;
}) => {
  const docker_organizations = useRead("ListDockerOrganizations", {}).data;
  return (
    <ConfigItem label="Docker Organization">
      <Select
        value={value}
        onValueChange={(value) => set({ docker_organization: value })}
      >
        <SelectTrigger className="w-full lg:w-[300px] max-w-[50%]">
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
