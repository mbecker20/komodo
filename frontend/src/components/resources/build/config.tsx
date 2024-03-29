import { Config } from "@components/config";
import {
  AccountSelector,
  ConfigItem,
  ResourceSelector,
} from "@components/config/util";
import { useRead, useWrite } from "@lib/hooks";
import { env_to_text, text_to_env } from "@lib/utils";
import { Types } from "@monitor/client";
import { Textarea } from "@ui/textarea";
import { useEffect, useState } from "react";

export const BuildConfig = ({ id }: { id: string }) => {
  const config = useRead("GetBuild", { build: id }).data?.config;
  // const orgs = useRead("GetAccounts")
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
            use_buildx: true,
            // docker_organization,
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
