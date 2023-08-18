import { AccountSelector, ResourceSelector } from "@components/config/util";
import { useWrite, useRead } from "@hooks";
import { ConfigInner } from "@layouts/page";
import { Types } from "@monitor/client";
import { useState } from "react";

export const BuildConfig = ({ id }: { id: string }) => {
  const config = useRead("GetBuild", { id }).data?.config;
  const [update, set] = useState<Partial<Types.BuildConfig>>({});
  const { mutate } = useWrite("UpdateBuild");

  if (!config) return null;

  return (
    <ConfigInner
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
        },
        docker: {
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
      }}
    />
  );
};
