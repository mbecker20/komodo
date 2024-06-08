import { Config } from "@components/config";
import { AccountSelector, ConfigItem } from "@components/config/util";
import { useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { ReactNode, useState } from "react";
import { CopyGithubWebhook } from "../common";

export const ResourceSyncConfig = ({
  id,
  titleOther,
}: {
  id: string;
  titleOther: ReactNode;
}) => {
  const perms = useRead("GetPermissionLevel", {
    target: { type: "ResourceSync", id },
  }).data;
  const config = useRead("GetResourceSync", { sync: id }).data?.config;
  const global_disabled =
    useRead("GetCoreInfo", {}).data?.ui_write_disabled ?? false;
  const [update, set] = useState<Partial<Types.ResourceSyncConfig>>({});
  const { mutateAsync } = useWrite("UpdateResourceSync");
  if (!config) return null;

  const disabled = global_disabled || perms !== Types.PermissionLevel.Write;

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
              repo: { placeholder: "Enter repo" },
              branch: { placeholder: "Enter branch" },
              commit: { placeholder: "Enter specific commit hash. Optional." },
              github_account: (value, set) => {
                return (
                  <AccountSelector
                    account_type="github"
                    type="None"
                    selected={value}
                    onSelect={(github_account) => set({ github_account })}
                    disabled={disabled}
                    placeholder="None"
                  />
                );
              },
              resource_path: { placeholder: "./resources" },
              delete: { label: "Delete Unmatched Resources" },
            },
          },
          {
            label: "Github Webhook",
            components: {
              ["refresh" as any]: () => (
                <ConfigItem label="Refresh Pending">
                  <CopyGithubWebhook path={`/sync/${id}/refresh`} />
                </ConfigItem>
              ),
              ["execute" as any]: () => (
                <ConfigItem label="Execute Sync">
                  <CopyGithubWebhook path={`/sync/${id}/execute`} />
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
