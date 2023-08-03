import { useServerStats } from "@hooks";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@ui/card";
import { Cpu, Database, LineChart, MemoryStick } from "lucide-react";
import { useParams } from "react-router-dom";

export const ServerStats = () => {
  const server_id = useParams().serverId;
  if (!server_id) return null;

  return (
    <div className="flex flex-col">
      <div className="flex justify-between">
        <div className="flex items-center gap-2 text-muted-foreground">
          <LineChart className="w-4 h-4" />
          <h2 className="text-xl">Server Stats</h2>
        </div>
        {/* <Link to={`/deployments/${id}/updates`}>
          <Button variant="secondary">
            <ExternalLink className="w-4 h-4" />
          </Button>
        </Link> */}
      </div>
      <div className="flex flex-col lg:flex-row gap-4 mt-2">
        <CPU server_id={server_id} />
        <RAM server_id={server_id} />
        <DISK server_id={server_id} />
        <LOAD server_id={server_id} />
      </div>
    </div>
  );
};

const CPU = ({ server_id }: { server_id: string }) => {
  const stats = useServerStats(server_id);

  let perc = stats?.cpu.cpu_perc;
  return (
    <Card className="w-full">
      <CardHeader className="flex-row justify-between">
        <CardTitle>CPU Usage</CardTitle>
        <div className="flex gap-2 items-center">
          <CardDescription>{perc?.toFixed(2)}%</CardDescription>
          <Cpu className="w-4 h-4" />
        </div>
      </CardHeader>
      <CardContent>
        <div className="relative w-full rounded-lg bg-muted-foreground h-4">
          <div
            className="bg-blue-500 rounded-lg h-4 absolute left-0 top-0"
            style={{ width: `${perc}%` }}
          />
        </div>
      </CardContent>
    </Card>
  );
};

const RAM = ({ server_id }: { server_id: string }) => {
  const stats = useServerStats(server_id);
  let used = stats?.basic.mem_used_gb;
  let total = stats?.basic.mem_total_gb;

  let perc = ((used ?? 0) / (total ?? 0)) * 100;

  return (
    <Card className="w-full">
      <CardHeader className="flex-row justify-between">
        <CardTitle>RAM Usage</CardTitle>
        <div className="flex gap-2 items-center">
          <CardDescription>{perc.toFixed(2)}%</CardDescription>
          <MemoryStick className="w-4 h-4" />
        </div>
      </CardHeader>
      <CardContent>
        {!isNaN(perc) && (
          <div
            className="bg-blue-500 rounded-lg h-4"
            style={{ width: `${perc}%` }}
          />
        )}
      </CardContent>
    </Card>
  );
};

const DISK = ({ server_id }: { server_id: string }) => {
  const stats = useServerStats(server_id);
  let used = stats?.disk.used_gb;
  let total = stats?.disk.total_gb;

  let perc = ((used ?? 0) / (total ?? 0)) * 100;

  return (
    <Card className="w-full">
      <CardHeader className="flex-row justify-between">
        <CardTitle>Disk Usage</CardTitle>
        <div className="flex gap-2 items-center">
          <CardDescription>{perc?.toFixed(2)}%</CardDescription>
          <Database className="w-4 h-4" />
        </div>
      </CardHeader>
      <CardContent>
        {!isNaN(perc) && (
          <div
            className="bg-blue-500 rounded-lg h-4"
            style={{ width: `${perc}%` }}
          />
        )}
      </CardContent>
    </Card>
  );
};

const LOAD = ({ server_id }: { server_id: string }) => {
  const stats = useServerStats(server_id);
  let perc = stats?.basic.system_load;

  return (
    <Card className="w-full">
      <CardHeader>
        <CardTitle>System Load</CardTitle>
      </CardHeader>
      <CardContent>
        <div
          className="bg-blue-500 rounded-lg h-4"
          style={{ width: `${perc}%` }}
        />
      </CardContent>
    </Card>
  );
};
