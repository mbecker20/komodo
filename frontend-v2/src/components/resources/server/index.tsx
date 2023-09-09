import { useRead, useWrite } from "@lib/hooks";
import { cn } from "@lib/utils";
import { Types } from "@monitor/client";
import { RequiredResourceComponents } from "@types";
import { MapPin, Cpu, MemoryStick, Database, ServerIcon } from "lucide-react";
import { ServerStats } from "./stats";
import { ConfigInner } from "@components/config";
import { useState } from "react";

export const useServer = (id?: string) =>
  useRead("ListServers", {}).data?.find((d) => d.id === id);

export const Server: RequiredResourceComponents = {
  Name: ({ id }) => <>{useServer(id)?.name}</>,
  Description: ({ id }) => <>{useServer(id)?.info.status}</>,
  Info: ({ id }) => {
    const server = useServer(id);
    const stats = useRead(
      "GetBasicSystemStats",
      { server_id: id },
      { enabled: server ? server.info.status !== "Disabled" : false }
    ).data;
    const info = useRead(
      "GetSystemInformation",
      { server_id: id },
      { enabled: server ? server.info.status !== "Disabled" : false }
    ).data;
    return (
      <>
        <div className="flex items-center gap-2">
          <MapPin className="w-4 h-4" />
          {useServer(id)?.info.region}
        </div>
        <div className="flex gap-4 text-muted-foreground">
          <div className="flex gap-2 items-center">
            <Cpu className="w-4 h-4" />
            {info?.core_count ?? "N/A"} Core(s)
          </div>
          <div className="flex gap-2 items-center">
            <MemoryStick className="w-4 h-4" />
            {stats?.mem_total_gb.toFixed(2) ?? "N/A"} GB
          </div>
          <div className="flex gap-2 items-center">
            <Database className="w-4 h-4" />
            {stats?.disk_total_gb.toFixed(2) ?? "N/A"} GB
          </div>
        </div>
      </>
    );
  },
  Actions: () => null,
  Icon: ({ id }) => {
    const status = useServer(id)?.info.status;
    return (
      <ServerIcon
        className={cn(
          "w-4 h-4",
          status === Types.ServerStatus.Ok && "fill-green-500",
          status === Types.ServerStatus.NotOk && "fill-red-500",
          status === Types.ServerStatus.Disabled && "fill-blue-500"
        )}
      />
    );
  },
  Page: {
    Stats: ({ id }) => <ServerStats id={id} />,
    Config: ({ id }: { id: string }) => {
      const config = useRead("GetServer", { id }).data?.config;
      const [update, set] = useState<Partial<Types.ServerConfig>>({});
      const { mutate } = useWrite("UpdateServer");
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
                address: true,
                region: true,
                enabled: true,
                auto_prune: true,
              },
            },
            warnings: {
              cpu: {
                cpu_warning: true,
                cpu_critical: true,
              },
              memory: {
                mem_warning: true,
                mem_critical: true,
              },
              disk: {
                disk_warning: true,
                disk_critical: true,
              },
            },
          }}
        />
      );
    },
  },
};
