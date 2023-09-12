import { ConfigInner } from "@components/config";
import { AccountSelector, ResourceSelector } from "@components/config/util";
import { useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { RequiredResourceComponents } from "@types";
import { GitBranch } from "lucide-react";
import { useState } from "react";

const useRepo = (id?: string) =>
  useRead("ListRepos", {}).data?.find((d) => d.id === id);

export const Repo: RequiredResourceComponents = {
  Name: ({ id }) => <>{useRepo(id)?.name}</>,
  Description: ({ id }) => <>{id}</>,
  Info: ({ id }) => <>{id}</>,
  Icon: () => <GitBranch className="w-4 h-4" />,
  Page: {
    Config: ({ id }) => {
      const config = useRead("GetRepo", { id }).data?.config;
      const [update, set] = useState<Partial<Types.RepoConfig>>({});
      const mutate = useWrite("UpdateRepo");
      if (!config) return null;
      return (
        <ConfigInner
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
    },
  },
  Actions: () => null,
  New: () => null,
};
