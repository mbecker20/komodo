import { Config } from "@components/config";
import { InputList } from "@components/config/util";
import { useInvalidate, useRead, useWrite } from "@lib/hooks";
import { Types } from "@komodo/client";
import { Button } from "@ui/button";
import { PlusCircle } from "lucide-react";
import { ReactNode, useState } from "react";
import { Switch } from "@ui/switch";

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
      resource_id={id}
      resource_type="Server"
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
                placeholder: "https://12.34.56.78:8120",
                description:
                  "The http/s address of periphery in your network, eg. https://12.34.56.78:8120",
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
          {
            label: "Ignore Disks",
            contentHidden:
              (update.ignore_mounts ?? config.ignore_mounts)?.length === 0,
            description:
              "If undesired mount points are coming through in server stats, filter them out here.",
            actions: !disabled && (
              <Button
                variant="secondary"
                onClick={() =>
                  set((update) => ({
                    ...update,
                    ignore_mounts: [
                      ...(update.ignore_mounts ?? config.ignore_mounts ?? []),
                      "",
                    ],
                  }))
                }
                className="flex items-center gap-2 w-[200px]"
              >
                <PlusCircle className="w-4 h-4" />
                Ignore Disk
              </Button>
            ),
            components: {
              ignore_mounts: (values, set) => (
                <InputList
                  field="ignore_mounts"
                  values={values ?? []}
                  set={set}
                  disabled={disabled}
                  placeholder="/path/to/disk"
                />
              ),
            },
          },
          {
            label: "Send Unreachable Alerts",
            contentHidden: true,
            description:
              "Send an alert if the Periphery agent cannot be reached.",
            actions: (
              <Switch
                checked={
                  update.send_unreachable_alerts ??
                  config.send_unreachable_alerts
                }
                onCheckedChange={(send_unreachable_alerts) =>
                  set({ ...update, send_unreachable_alerts })
                }
              />
            ),
            components: {},
          },
          {
            label: "CPU Alerts",
            labelHidden: true,
            components: {
              send_cpu_alerts: {
                label: "Send CPU Alerts",
                boldLabel: true,
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
            labelHidden: true,
            components: {
              send_mem_alerts: {
                label: "Send Memory Alerts",
                boldLabel: true,
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
            labelHidden: true,
            components: {
              send_disk_alerts: {
                boldLabel: true,
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
