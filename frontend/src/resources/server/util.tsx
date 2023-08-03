import { useRead } from "@hooks";
import { ServerStatus } from "@monitor/client/dist/types";
import { CardDescription } from "@ui/card";
import { cn } from "@util/helpers";
import { Cpu, Database, MapPin, MemoryStick, Server } from "lucide-react";

export const ServerName = ({ serverId }: { serverId: string | undefined }) => {
  const servers = useRead("ListServers", {}).data;
  const server = servers?.find((s) => s.id === serverId);
  return <>{server?.name ?? "..."}</>;
};

export const ServerInfo = ({ serverId }: { serverId: string | undefined }) => {
  const servers = useRead("ListServers", {}).data;
  const server = servers?.find((s) => s.id === serverId);
  return (
    <div className="flex items-center gap-4 text-muted-foreground">
      <div className="flex items-center gap-2">
        <ServerStatusIcon serverId={serverId} />
        <div> {server?.status}</div>
      </div>
      <CardDescription className="hidden md:block">|</CardDescription>
      {serverId && <ServerSpecs server_id={serverId} />}
    </div>
  );
};

export const ServerSpecs = ({ server_id }: { server_id: string }) => {
  const stats = useRead("GetBasicSystemStats", { server_id }).data;
  const info = useRead("GetSystemInformation", { server_id }).data;

  return (
    <div className="flex gap-4 text-muted-foreground">
      <div className="flex gap-2 items-center">
        <Cpu className="w-4 h-4" />
        {info?.core_count} {`Core${(info?.core_count ?? 1) > 1 ? "s" : ""}`}
      </div>
      <div className="flex gap-2 items-center">
        <MemoryStick className="w-4 h-4" />
        {stats?.mem_total_gb.toFixed(2)} GB
      </div>
      <div className="flex gap-2 items-center">
        <Database className="w-4 h-4" />
        {stats?.disk_total_gb.toFixed(2)} GB
      </div>
    </div>
  );
};

export const ServerRegion = () => {
  return (
    <div className="flex gap-2 items-center text-muted-foreground">
      <MapPin className="w-4 h-4" />
      server.region
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
  const servers = useRead("ListServers", {}).data;
  const server = servers?.find((s) => s.id === serverId);
  return (
    <Server
      className={cn(
        "w-4 h-4 stroke-primary",
        server?.status === ServerStatus.Ok && "fill-green-500",
        server?.status === ServerStatus.NotOk && "fill-red-500",
        server?.status === ServerStatus.Disabled && "fill-blue-500",
        sm && "w-3 h-3"
      )}
    />
  );
};
