import { Section } from "@components/layouts";
import { client } from "@main";
import { Types } from "@monitor/client";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@ui/card";
import { Progress } from "@ui/progress";
import { Cpu, Database, LineChart, MemoryStick } from "lucide-react";
import { useState, useEffect, useCallback } from "react";
import { useServer } from ".";

const useServerStats = (server_id: string) => {
  const [stats, set] = useState<Types.AllSystemStats>();
  const server = useServer(server_id);

  const fetch = useCallback(
    () =>
      !!server &&
      server.info.status !== "Disabled" &&
      client
        .read({ type: "GetAllSystemStats", params: { server_id: server.id } })
        .then(set),
    [server]
  );

  useEffect(() => {
    fetch();
    if (!!server && server.info.status !== "Disabled") {
      const handle = setInterval(() => {
        fetch();
      }, 1000);
      return () => {
        clearInterval(handle);
      };
    }
  }, [server, fetch]);

  return stats;
};

export const ServerStats = ({ id }: { id: string }) => {
  return (
    <Section
      title="Server Stats"
      icon={<LineChart className="w-4 h-4" />}
      actions=""
    >
      <div className="flex flex-col lg:flex-row gap-4">
        <CPU id={id} />
        <RAM id={id} />
        <DISK id={id} />
      </div>
    </Section>
  );
};

const CPU = ({ id }: { id: string }) => {
  const stats = useServerStats(id);

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

const RAM = ({ id }: { id: string }) => {
  const stats = useServerStats(id);
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

const DISK = ({ id }: { id: string }) => {
  const stats = useServerStats(id);
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
