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
              address: {
                placeholder: "http://localhost:8120",
                description:
                  "The http/s address of periphery in your network, eg. http://12.34.56.78:8120",
              },
              region: {
                placeholder: "Region. Optional.",
                description:
                  "Attach a region to the server for visual grouping.",
              },
              enabled: {
                description:
                  "Whether to attempt to connect to this host / send alerts if offline. Disabling will also convert all attached resource's state to 'Unknown'.",
              },
              stats_monitoring: {
                description:
                  "Whether to store historical CPU, RAM, and disk usage.",
              },
              auto_prune: {
                description:
                  "Whether to prune unused images every day at UTC 00:00",
              },
            },
          },
        ],
        alerts: [
          {
            label: "Unreachable Alert",
            labelHidden: true,
            components: {
              send_unreachable_alerts: {
                description:
                  "Send an alert if the server could not pass a basic health check. Configure 'Alerter' resources to route these.",
                boldLabel: true,
              },
            },
          },
          {
            label: "CPU Alerts",
            components: {
              send_cpu_alerts: {
                label: "Send CPU Alerts",
                description:
                  "Send an alert if the CPU usage is above the configured thresholds.",
              },
              cpu_warning: {
                description:
                  "Send a 'Warning' alert if the CPU usage in % is above these thresholds",
              },
              cpu_critical: {
                description:
                  "Send a 'Critical' alert if the CPU usage in % is above these thresholds",
              },
            },
          },
          {
            label: "Memory Alerts",
            components: {
              send_mem_alerts: {
                label: "Send Memory Alerts",
                description:
                  "Send an alert if the memory usage is above the configured thresholds.",
              },
              mem_warning: {
                description:
                  "Send a 'Warning' alert if the memory usage in % is above these thresholds",
              },
              mem_critical: {
                description:
                  "Send a 'Critical' alert if the memory usage in % is above these thresholds",
              },
            },
          },
          {
            label: "Disk Alerts",
            components: {
              send_disk_alerts: {
                description:
                  "Send an alert if the Disk Usage (for any mounted disk) is above the configured thresholds.",
              },
              disk_warning: {
                description:
                  "Send a 'Warning' alert if the disk usage in % is above these thresholds",
              },
              disk_critical: {
                description:
                  "Send a 'Critical' alert if the disk usage in % is above these thresholds",
              },
            },
          },
        ],
      }}
    />
  );
};
