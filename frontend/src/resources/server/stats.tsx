import { useServerStats } from "@hooks";
import { Section } from "@layouts/page";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@ui/card";
import { Progress } from "@ui/progress";
import { Cpu, Database, LineChart, MemoryStick } from "lucide-react";
import { useParams } from "react-router-dom";

export const ServerStats = () => {
  const server_id = useParams().serverId;
  if (!server_id) return null;

  return (
    <Section
      title="Server Stats"
      icon={<LineChart className="w-4 h-4" />}
      actions=""
    >
      <div className="flex flex-col lg:flex-row gap-4">
        <CPU server_id={server_id} />
        <RAM server_id={server_id} />
        <DISK server_id={server_id} />
      </div>
    </Section>
  );
};

const CPU = ({ server_id }: { server_id: string }) => {
  const stats = useServerStats(server_id);

  const perc = stats?.cpu.cpu_perc;
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
        <Progress value={perc} className="h-4" />
      </CardContent>
    </Card>
  );
};

const RAM = ({ server_id }: { server_id: string }) => {
  const stats = useServerStats(server_id);
  const used = stats?.basic.mem_used_gb;
  const total = stats?.basic.mem_total_gb;

  const perc = ((used ?? 0) / (total ?? 0)) * 100;

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
        <Progress value={perc} className="h-4" />
      </CardContent>
    </Card>
  );
};

const DISK = ({ server_id }: { server_id: string }) => {
  const stats = useServerStats(server_id);
  const used = stats?.disk.used_gb;
  const total = stats?.disk.total_gb;

  const perc = ((used ?? 0) / (total ?? 0)) * 100;

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
        <Progress value={perc} className="h-4" />
      </CardContent>
    </Card>
  );
};
