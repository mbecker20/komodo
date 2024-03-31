import { useRead } from "@lib/hooks";
import { useServer } from ".";
import { Cpu, Database, MapPin, MemoryStick } from "lucide-react";

export const ServerInfo = ({
  id,
  showRegion = true,
}: {
  id: string;
  showRegion?: boolean;
}) => {
  const server = useServer(id);
  const stats = useRead(
    "GetBasicSystemStats",
    { server: id },
    { enabled: server ? server.info.status !== "Disabled" : false }
  ).data;
  const info = useRead(
    "GetSystemInformation",
    { server: id },
    { enabled: server ? server.info.status !== "Disabled" : false }
  ).data;
  return (
    <>
      {showRegion && (
        <>
          <div className="flex items-center gap-2">
            <MapPin className="w-4 h-4" />
            {useServer(id)?.info.region}
          </div>
          |
        </>
      )}
      <div className="flex gap-2 items-center">
        <Cpu className="w-4 h-4" />
        {info?.core_count ?? "N/A"} Core(s)
      </div>
      |
      <div className="flex gap-2 items-center">
        <MemoryStick className="w-4 h-4" />
        {stats?.mem_total_gb.toFixed(2) ?? "N/A"} GB
      </div>
      |
      <div className="flex gap-2 items-center">
        <Database className="w-4 h-4" />
        {stats?.disk_total_gb.toFixed(2) ?? "N/A"} GB
      </div>
    </>
  );
};
