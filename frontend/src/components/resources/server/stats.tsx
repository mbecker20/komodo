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
import { ServerComponents, useServer } from ".";
import { DataTable } from "@ui/data-table";
import { Fragment, useState } from "react";
import { Input } from "@ui/input";
import { ResourceDescription } from "../common";
import { AddTags, ResourceTags } from "@components/tags";
import { StatChart } from "./stat-chart";
import { useStatsGranularity } from "./hooks";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";

export const ServerStats = ({ id }: { id: string }) => {
  const [interval, setInterval] = useStatsGranularity();

  const server = useServer(id);
  const stats = useRead(
    "GetSystemStats",
    { server: id },
    { refetchInterval: 5000 }
  ).data;
  const info = useRead("GetSystemInformation", { server: id }).data;

  const disk_used = stats?.disks.reduce(
    (acc, curr) => (acc += curr.used_gb),
    0
  );
  const disk_total = stats?.disks.reduce(
    (acc, curr) => (acc += curr.total_gb),
    0
  );

  return (
    <Page
      title={server?.name}
      titleRight={
        <div className="flex gap-4 items-center">
          {Object.entries(ServerComponents.Status).map(([key, Status]) => (
            <Status key={key} id={id} />
          ))}
        </div>
      }
      subtitle={
        <div className="flex flex-col gap-4">
          <div className="flex gap-4 items-center text-muted-foreground">
            <ServerComponents.Icon id={id} />
            {Object.entries(ServerComponents.Info).map(([key, Info], i) => (
              <Fragment key={key}>
                | <Info key={i} id={id} />
              </Fragment>
            ))}
          </div>
          <ResourceDescription type="Server" id={id} />
        </div>
      }
      actions={
        <div className="flex gap-2 items-center">
          <div className="text-muted-foreground">tags:</div>
          <ResourceTags
            target={{ id, type: "Server" }}
            className="text-sm"
            click_to_delete
          />
          <AddTags target={{ id, type: "Server" }} />
        </div>
      }
    >
      <Section title="System Info">
        <DataTable
          tableKey="system-info"
          data={
            info
              ? [{ ...info, mem_total: stats?.mem_total_gb, disk_total }]
              : []
          }
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
            {
              header: "Core Count",
              accessorFn: ({ core_count }) =>
                `${core_count} Core${(core_count || 0) > 1 ? "s" : ""}`,
            },
            {
              header: "Total Memory",
              accessorFn: ({ mem_total }) => `${mem_total?.toFixed(2)} GB`,
            },
            {
              header: "Total Disk Size",
              accessorFn: ({ disk_total }) => `${disk_total?.toFixed(2)} GB`,
            },
          ]}
        />
      </Section>

      <Section title="Current">
        <div className="flex flex-col lg:flex-row gap-4">
          <CPU stats={stats} />
          <RAM stats={stats} />
          <DISK stats={stats} />
        </div>
      </Section>

      <Section
        title="Historical"
        actions={
          <div className="flex gap-2 items-center">
            <div className="text-muted-foreground">Interval:</div>
            <Select
              value={interval}
              onValueChange={(interval) =>
                setInterval(interval as Types.Timelength)
              }
            >
              <SelectTrigger className="w-[150px]">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {[
                  Types.Timelength.FifteenSeconds,
                  Types.Timelength.ThirtySeconds,
                  Types.Timelength.OneMinute,
                  Types.Timelength.FiveMinutes,
                  Types.Timelength.FifteenMinutes,
                  Types.Timelength.ThirtyMinutes,
                  Types.Timelength.OneHour,
                  Types.Timelength.SixHours,
                  Types.Timelength.OneDay,
                ].map((timelength) => (
                  <SelectItem value={timelength}>{timelength}</SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
        }
      >
        <div className="flex flex-col gap-8">
          <StatChart server_id={id} type="cpu" className="w-full h-[250px]" />
          <StatChart server_id={id} type="mem" className="w-full h-[250px]" />
          <StatChart server_id={id} type="disk" className="w-full h-[250px]" />
        </div>
      </Section>

      <Section
        title="Disks"
        actions={
          <div className="flex gap-4 items-center">
            <div className="flex gap-2 items-center">
              <div className="text-muted-foreground">Used:</div>
              {disk_used?.toFixed(2)} GB
            </div>
            <div className="flex gap-2 items-center">
              <div className="text-muted-foreground">Total:</div>
              {disk_total?.toFixed(2)} GB
            </div>
          </div>
        }
      >
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
