import { Config } from "@components/config";
import { ConfigItem, SystemCommand } from "@components/config/util";
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
import { CopyGithubWebhook, ServerSelector } from "../common";

export const RepoConfig = ({ id }: { id: string }) => {
  const perms = useRead("GetPermissionLevel", {
    target: { type: "Repo", id },
  }).data;
  const config = useRead("GetRepo", { repo: id }).data?.config;
  const [update, set] = useState<Partial<Types.RepoConfig>>({});
  const { mutateAsync } = useWrite("UpdateRepo");
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
          "": {
            server_id: (value, set) => (
              <ServerSelector selected={value} set={set} disabled={disabled} />
            ),
          },
          general: {
            repo: true,
            branch: true,
            github_account: (value, set) => {
              const server_id = update.server_id || config.server_id;
              if (server_id) {
                return (
                  <GithubAccount
                    server={server_id}
                    value={value}
                    set={set}
                    disabled={disabled}
                  />
                );
              }
            },
            on_clone: (value, set) => (
              <SystemCommand
                label="On Clone"
                value={value}
                set={(value) => set({ on_clone: value })}
                disabled={disabled}
              />
            ),
            on_pull: (value, set) => (
              <SystemCommand
                label="On Pull"
                value={value}
                set={(value) => set({ on_pull: value })}
                disabled={disabled}
              />
            ),
          },
          github_webhooks: {
            ["clone" as any]: () => (
              <ConfigItem label="Clone">
                <CopyGithubWebhook path={`/repo/${id}/clone`} />
              </ConfigItem>
            ),
            ["pull" as any]: () => (
              <ConfigItem label="Pull">
                <CopyGithubWebhook path={`/repo/${id}/pull`} />
              </ConfigItem>
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
  disabled,
}: {
  value?: string;
  set: (input: Partial<Types.RepoConfig>) => void;
  server: string;
  disabled: boolean;
}) => {
  const accounts = useRead("GetAvailableAccounts", {
    server,
  }).data;
  return (
    <ConfigItem label="Github Account">
      <Select
        value={value}
        onValueChange={(value) => set({ github_account: value })}
        disabled={disabled}
      >
        <SelectTrigger
          className="w-full lg:w-[300px] max-w-[50%]"
          disabled={disabled}
        >
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
