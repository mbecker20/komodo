import { Config } from "@components/config";
import { AccountSelector, ResourceSelector } from "@components/config/util";
import { useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { useState } from "react";

export const RepoConfig = ({ id }: { id: string }) => {
  const config = useRead("GetRepo", { repo: id }).data?.config;
  const [update, set] = useState<Partial<Types.RepoConfig>>({});
  const mutate = useWrite("UpdateRepo");
  if (!config) return null;
  return (
    <Config
      config={config}
      update={update}
      set={set}
      onSave={() => mutate}
      components={{
        general: {
          general: {
            server_id: (selected, set) => (
              <ResourceSelector
                type="Server"
                selected={selected}
                onSelect={(server_id) => set({ server_id })}
              />
            ),
            github_account: (value, set) => (
              <AccountSelector
                type="Server"
                account_type="github"
                id={update.server_id ?? config.server_id}
                selected={value}
                onSelect={(github_account) => set({ github_account })}
              />
            ),
            repo: true,
            branch: true,
            on_pull: true,
            on_clone: true,
          },
        },
      }}
    />
  );
};
