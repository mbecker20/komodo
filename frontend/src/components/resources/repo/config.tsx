import { Config } from "@components/config";
import {
  AccountSelector,
  ConfigItem,
  SystemCommand,
} from "@components/config/util";
import { useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
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
        general: [
          {
            label: "Server Id",
            labelHidden: true,
            components: {
              server_id: (value, set) => (
                <ServerSelector
                  selected={value}
                  set={set}
                  disabled={disabled}
                />
              ),
            },
          },
          {
            label: "General",
            components: {
              repo: { placeholder: "Enter repo" },
              branch: { placeholder: "Enter branch" },
              commit: { placeholder: "Enter specific commit hash. Optional." },
              github_account: (value, set) => {
                const server_id = update.server_id || config.server_id;
                if (server_id) {
                  return (
                    <AccountSelector
                      id={server_id}
                      account_type="github"
                      type="Server"
                      selected={value}
                      onSelect={(github_account) => set({ github_account })}
                      disabled={disabled}
                      placeholder="None"
                    />
                  );
                }
              },
            },
          },
          {
            label: "On Clone",
            components: {
              on_clone: (value, set) => (
                <SystemCommand
                  value={value}
                  set={(value) => set({ on_clone: value })}
                  disabled={disabled}
                />
              ),
            },
          },
          {
            label: "On Pull",
            components: {
              on_pull: (value, set) => (
                <SystemCommand
                  value={value}
                  set={(value) => set({ on_pull: value })}
                  disabled={disabled}
                />
              ),
            },
          },
          {
            label: "Github Webhooks",
            components: {
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
              webhook_enabled: true,
            },
          },
        ],
      }}
    />
  );
};
