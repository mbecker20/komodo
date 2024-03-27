import { Section } from "@components/layouts";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@ui/card";
import { Progress } from "@ui/progress";
import { Cpu, Database, LineChart, MemoryStick } from "lucide-react";
import { useRead } from "@lib/hooks";
import { Types } from "@monitor/client";

export const ServerStats = ({ server_id }: { server_id: string }) => {
  const stats = useRead(
    "GetAllSystemStats",
    { server: server_id },
    { refetchInterval: 1000 }
  ).data;

  return (
    <Section
      title="Server Stats"
      icon={<LineChart className="w-4 h-4" />}
      actions=""
    >
      <div className="flex flex-col lg:flex-row gap-4">
        <CPU stats={stats} />
        <RAM stats={stats} />
        <DISK stats={stats} />
      </div>
    </Section>
  );
};

const CPU = ({ stats }: { stats: Types.AllSystemStats | undefined }) => {
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

const RAM = ({ stats }: { stats: Types.AllSystemStats | undefined }) => {
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

const DISK = ({ stats }: { stats: Types.AllSystemStats | undefined }) => {
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
