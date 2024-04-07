import { Config } from "@components/config";
import {
  ConfigItem,
  ResourceSelector,
  SystemCommand,
} from "@components/config/util";
import { useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";
import { useState } from "react";

export const RepoConfig = ({ id }: { id: string }) => {
  const config = useRead("GetRepo", { repo: id }).data?.config;
  const [update, set] = useState<Partial<Types.RepoConfig>>({});
  const { mutate } = useWrite("UpdateRepo");
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
            server_id: (selected, set) => (
              <ConfigItem label="Server">
                <ResourceSelector
                  type="Server"
                  selected={selected}
                  onSelect={(server_id) => set({ server_id })}
                />
              </ConfigItem>
            ),
            repo: true,
            branch: true,
            github_account: (value, set) => {
              const server_id = update.server_id || config.server_id;
              if (server_id) {
                return (
                  <GithubAccount server={server_id} value={value} set={set} />
                );
              }
            },
            on_clone: (value, set) => (
              <SystemCommand
                label="On Clone"
                value={value}
                set={(value) => set({ on_clone: value })}
              />
            ),
            on_pull: (value, set) => (
              <SystemCommand
                label="On Pull"
                value={value}
                set={(value) => set({ on_pull: value })}
              />
            ),
          },
        },
      }}
    />
  );
};

const GithubAccount = ({
  value,
  set,
  server,
}: {
  value?: string;
  set: (input: Partial<Types.RepoConfig>) => void;
  server: string;
}) => {
  const accounts = useRead("GetAvailableAccounts", {
    server,
  }).data;
  return (
    <ConfigItem label="Github Account">
      <Select
        value={value}
        onValueChange={(value) => set({ github_account: value })}
      >
        <SelectTrigger className="w-full lg:w-[300px] max-w-[50%]">
          <SelectValue placeholder="Select Account" />
        </SelectTrigger>
        <SelectContent>
          {accounts?.github?.map((account: string) => (
            <SelectItem key={account} value={account}>
              {account}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </ConfigItem>
  );
};
