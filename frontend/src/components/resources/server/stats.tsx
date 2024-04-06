import { Page, Section } from "@components/layouts";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@ui/card";
import { Progress } from "@ui/progress";
import { Cpu, Database, MemoryStick } from "lucide-react";
import { useRead } from "@lib/hooks";
import { Types } from "@monitor/client";
import { useServer } from ".";
import { ServerInfo } from "./info";
import { DataTable } from "@ui/data-table";

export const ServerStats = ({ id }: { id: string }) => {
  const server = useServer(id);
  const stats = useRead(
    "GetAllSystemStats",
    { server: id },
    { refetchInterval: 5000 }
  ).data;
  const info = useRead("GetSystemInformation", { server: id }).data;

  return (
    <Page
      title={server?.name}
      titleRight={<div className="text-muted-foreground">Stats</div>}
    >
      <div className="flex gap-4">
        <ServerInfo id={id} />
      </div>
      <Section title="System Info">
        <DataTable
          data={info && [info]}
          columns={[
            {
              header: "Hostname",
              accessorKey: "host_name",
            },
            {
              header: "Os",
              accessorKey: "os",
            },
            {
              header: "Kernel",
              accessorKey: "kernel",
            },
          ]}
        />
      </Section>
      <Section title="Basic">
        <div className="flex flex-col lg:flex-row gap-4">
          <CPU stats={stats} />
          <RAM stats={stats} />
          <DISK stats={stats} />
        </div>
      </Section>
      <Section title="Disks">
        <DataTable
          data={stats?.disk.disks}
          columns={[
            {
              header: "Path",
              accessorKey: "mount",
            },
            {
              header: "Used",
              accessorFn: (disk) => disk.used_gb.toFixed(2) + " GB",
            },
            {
              header: "Total",
              accessorFn: (disk) => disk.total_gb.toFixed(2) + " GB",
            },
            {
              header: "Percentage",
              accessorFn: (disk) =>
                (100 * (disk.used_gb / disk.total_gb)).toFixed(2) + "% Full",
            },
          ]}
        />
      </Section>
      {stats?.processes && stats.processes.length > 0 && (
        <Section title="Processes">
          <DataTable
            data={stats.processes}
            columns={[
              {
                header: "Name",
                accessorKey: "name",
              },
              {
                header: "Cpu",
                accessorFn: (process) => `${process.cpu_perc.toFixed(2)} %`,
              },
              {
                header: "Memory",
                accessorFn: (process) =>
                  process.mem_mb > 1000
                    ? `${(process.mem_mb / 1024).toFixed(2)} GB`
                    : `${process.mem_mb.toFixed(2)} MB`,
              },
            ]}
          />
        </Section>
      )}
    </Page>
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
