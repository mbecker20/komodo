import { useRead } from "@hooks";
import { ServerStatus } from "@monitor/client/dist/types";
import { CardDescription } from "@ui/card";
import { cn } from "@util/helpers";
import { Circle, Cpu, Database, MemoryStick } from "lucide-react";
import { useEffect } from "react";

export const ServerName = ({ serverId }: { serverId: string | undefined }) => {
  const servers = useRead({ type: "ListServers", params: {} }).data;
  const server = servers?.find((s) => s.id === serverId);
  return <>{server?.name ?? "..."}</>;
};

export const ServerInfo = ({ serverId }: { serverId: string | undefined }) => {
  const servers = useRead({ type: "ListServers", params: {} }).data;
  const server = servers?.find((s) => s.id === serverId);
  return (
    <div className="flex items-center gap-4">
      {serverId && <ServerStats serverId={serverId} />}
      <CardDescription>|</CardDescription>
      <div className="flex items-center gap-2">
        <CardDescription> Status: {server?.status}</CardDescription>
        <ServerStatusIcon serverId={serverId} />
      </div>
    </div>
  );
};

export const ServerStats = ({ serverId }: { serverId: string }) => {
  const { data, refetch } = useRead({
    type: "GetBasicSystemStats",
    params: { server_id: serverId },
  });

  useEffect(() => {
    const handle = setInterval(() => refetch(), 30000);
    return () => {
      clearInterval(handle);
    };
  }, [refetch]);

  return (
    <div className="flex gap-4">
      <CardDescription className="flex gap-2 items-center">
        <Cpu className="w-4 h-4" />
        {data?.cpu_perc.toFixed(2)}%
      </CardDescription>
      <CardDescription className="flex gap-2 items-center">
        <MemoryStick className="w-4 h-4" />
        {data?.mem_total_gb.toFixed(2)} GB
      </CardDescription>
      <CardDescription className="flex gap-2 items-center">
        <Database className="w-4 h-4" />
        {data?.disk_total_gb.toFixed(2)} GB
      </CardDescription>
    </div>
  );
};

export const ServerStatusIcon = ({
  serverId,
  sm,
}: {
  serverId: string | undefined;
  sm?: boolean;
}) => {
  const servers = useRead({ type: "ListServers", params: {} }).data;
  const server = servers?.find((s) => s.id === serverId);
  return (
    <Circle
      className={cn(
        "w-4 h-4 stroke-none",
        server?.status === ServerStatus.Ok && "fill-green-500",
        server?.status === ServerStatus.NotOk && "fill-red-500",
        server?.status === ServerStatus.Disabled && "fill-blue-500",
        sm && "w-3 h-3"
      )}
    />
  );
};
