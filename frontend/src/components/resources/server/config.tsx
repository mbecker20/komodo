import { Config } from "@components/config";
import { useInvalidate, useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { ReactNode, useState } from "react";

export const ServerConfig = ({
  id,
  titleOther,
}: {
  id: string;
  titleOther: ReactNode;
}) => {
  const perms = useRead("GetPermissionLevel", {
    target: { type: "Server", id },
  }).data;
  const invalidate = useInvalidate();
  const config = useRead("GetServer", { server: id }).data?.config;
  const global_disabled =
    useRead("GetCoreInfo", {}).data?.ui_write_disabled ?? false;
  const [update, set] = useState<Partial<Types.ServerConfig>>({});
  const { mutateAsync } = useWrite("UpdateServer", {
    onSuccess: () => {
      // In case of disabling to resolve unreachable alert
      invalidate(["ListAlerts"]);
    },
  });
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
              address: { placeholder: "http://localhost:8120. Required" },
              region: { placeholder: "Region. Optional." },
              enabled: true,
              stats_monitoring: true,
              auto_prune: true,
            },
          },
        ],
        alerts: [
          {
            label: "Alerts",
            components: {
              send_unreachable_alerts: true,
              send_cpu_alerts: true,
              send_disk_alerts: true,
              send_mem_alerts: true,
            },
          },
        ],
        warnings: [
          {
            label: "Cpu",
            components: {
              cpu_warning: true,
              cpu_critical: true,
            },
          },
          {
            label: "Memory",
            components: {
              mem_warning: true,
              mem_critical: true,
            },
          },
          {
            label: "Disk",
            components: {
              disk_warning: true,
              disk_critical: true,
            },
          },
        ],
      }}
    />
  );
};
