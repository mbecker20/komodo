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
import { DataTable } from "@ui/data-table";
import { useState } from "react";
import { Input } from "@ui/input";

export const ServerStats = ({ id }: { id: string }) => {
  const server = useServer(id);
  const stats = useRead(
    "GetSystemStats",
    { server: id },
    { refetchInterval: 5000 }
  ).data;
  const info = useRead("GetSystemInformation", { server: id }).data;

  return (
    <Page
      title={server?.name}
      titleRight={<div className="text-muted-foreground">Stats</div>}
    >
      <div className="flex gap-4">{/* <ServerInfo id={id} /> */}</div>

      <Section title="System Info">
        <DataTable
          tableKey="system-info"
          data={info ? [info] : []}
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

      {/* <Section title="Load">
        <div className="flex flex-col lg:flex-row gap-4">
          {["one", "five", "fifteen"].map((minutes) => (
            <LOAD
              load={stats?.basic.load_average}
              minutes={minutes as keyof Types.LoadAverage}
              core_count={info?.core_count || 0}
            />
          ))}
        </div>
      </Section> */}

      <Section title="Disks">
        <DataTable
          tableKey="server-disks"
          data={stats?.disks ?? []}
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

      <Processes id={id} />
    </Page>
  );
};

const Processes = ({ id }: { id: string }) => {
  const [search, setSearch] = useState("");
  const searchSplit = search.split(" ");

  const { data: processes } = useRead("GetSystemProcesses", { server: id });
  if (!processes || processes.length === 0) return;

  return (
    <Section
      title="Processes"
      actions={
        <Input
          placeholder="Search Processes"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          className="w-[300px]"
        />
      }
    >
      <DataTable
        tableKey="server-processes"
        data={processes.filter((process) =>
          searchSplit.every((search) => process.name.includes(search))
        )}
        columns={[
          {
            header: "Name",
            accessorKey: "name",
          },
          {
            header: "Exe",
            accessorKey: "exe",
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
  );
};

const CPU = ({ stats }: { stats: Types.SystemStats | undefined }) => {
  const perc = stats?.cpu_perc;

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

const RAM = ({ stats }: { stats: Types.SystemStats | undefined }) => {
  const used = stats?.mem_used_gb;
  const total = stats?.mem_total_gb;

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

const DISK = ({ stats }: { stats: Types.SystemStats | undefined }) => {
  const used = stats?.disks.reduce((acc, curr) => (acc += curr.used_gb), 0);
  const total = stats?.disks.reduce((acc, curr) => (acc += curr.total_gb), 0);

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
