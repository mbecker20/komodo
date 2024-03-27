import { useInvalidate, useRead, useWrite } from "@lib/hooks";
import { cn, fmt_date_with_minutes } from "@lib/utils";
import { Types } from "@monitor/client";
import { RequiredResourceComponents } from "@types";
import {
  MapPin,
  Cpu,
  MemoryStick,
  Database,
  ServerIcon,
  AlertTriangle,
  Rocket,
} from "lucide-react";
import { ServerStats } from "./stats";
import { ConfigInner } from "@components/config";
import { useState } from "react";
import { NewResource, Section } from "@components/layouts";
import { Input } from "@ui/input";
import { DataTable } from "@ui/data-table";
import { Link } from "react-router-dom";
import { ResourceComponents } from "..";
import { TagsWithBadge, useTagsFilter } from "@components/tags";
import { DeleteServer, RenameServer } from "./actions";
import { ServersChart } from "@components/dashboard/servers-chart";
import { DeploymentTable } from "../deployment";

export const useServer = (id?: string) =>
  useRead("ListServers", {}).data?.find((d) => d.id === id);

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
        <div className="flex items-center gap-2">
          <MapPin className="w-4 h-4" />
          {useServer(id)?.info.region}
        </div>
      )}
      <div className="flex gap-4 text-muted-foreground">
        <div className="flex gap-2 items-center">
          <Cpu className="w-4 h-4" />
          {info?.core_count ?? "N/A"} Core(s)
        </div>
        <div className="flex gap-2 items-center">
          <MemoryStick className="w-4 h-4" />
          {stats?.mem_total_gb.toFixed(2) ?? "N/A"} GB
        </div>
        <div className="flex gap-2 items-center">
          <Database className="w-4 h-4" />
          {stats?.disk_total_gb.toFixed(2) ?? "N/A"} GB
        </div>
      </div>
    </>
  );
};

export const ServerIconComponent = ({ id }: { id?: string }) => {
  const status = useServer(id)?.info.status;

  const color = () => {
    if (status === Types.ServerStatus.Ok) return "fill-green-500";
    if (status === Types.ServerStatus.NotOk) return "fill-red-500";
    if (status === Types.ServerStatus.Disabled) return "fill-blue-500";
  };
  return <ServerIcon className={cn("w-4 h-4", id && color())} />;
};

const ServerConfig = ({ id }: { id: string }) => {
  const invalidate = useInvalidate();
  const config = useRead("GetServer", { server: id }).data?.config;
  const [update, set] = useState<Partial<Types.ServerConfig>>({});
  const { mutate } = useWrite("UpdateServer", {
    onSuccess: () => {
      // In case of disabling to resolve unreachable alert
      invalidate(["ListAlerts"]);
    },
  });
  if (!config) return null;

  return (
    <ConfigInner
      config={config}
      update={update}
      set={set}
      onSave={() => mutate({ id, config: update })}
      components={{
        general: {
          general: {
            address: true,
            region: true,
            enabled: true,
            auto_prune: true,
          },
        },
        warnings: {
          cpu: {
            cpu_warning: true,
            cpu_critical: true,
          },
          memory: {
            mem_warning: true,
            mem_critical: true,
          },
          disk: {
            disk_warning: true,
            disk_critical: true,
          },
        },
      }}
    />
  );
};

const NewServer = () => {
  const { mutateAsync } = useWrite("CreateServer");
  const [name, setName] = useState("");
  return (
    <NewResource
      type="Server"
      onSuccess={() => mutateAsync({ name, config: {} })}
      enabled={!!name}
    >
      <div className="grid md:grid-cols-2">
        Server Name
        <Input
          placeholder="server-name"
          value={name}
          onChange={(e) => setName(e.target.value)}
        />
      </div>
    </NewResource>
  );
};

const DeploymentCountOnServer = ({ id }: { id: string }) => {
  const { data } = useRead("ListDeployments", {
    query: { specific: { server_ids: [id] } },
  });

  return <>{data?.length ?? 0}</>;
};

const ServerTable = () => {
  const servers = useRead("ListServers", {}).data;
  const tags = useTagsFilter();
  return (
    <DataTable
      // onRowClick={({ id }) => nav(`/servers/${id}`)}
      data={
        servers?.filter((server) =>
          tags.every((tag) => server.tags.includes(tag))
        ) ?? []
      }
      columns={[
        {
          header: "Name",
          accessorKey: "id",
          cell: ({
            row: {
              original: { id },
            },
          }) => {
            return (
              <Link to={`/servers/${id}`} className="flex gap-2">
                <ResourceComponents.Server.Icon id={id} />
                <ResourceComponents.Server.Name id={id} />
              </Link>
            );
          },
        },
        // {
        //   header: "Description",
        //   accessorKey: "description",
        // },

        {
          header: "Deployments",
          cell: ({ row }) => <DeploymentCountOnServer id={row.original.id} />,
        },
        { header: "Region", accessorKey: "info.region" },
        {
          header: "Tags",
          cell: ({ row }) => {
            return (
              <div className="flex gap-1">
                <TagsWithBadge tag_ids={row.original.tags} />
              </div>
            );
          },
        },
        {
          header: "Created",
          accessorFn: ({ created_at }) =>
            fmt_date_with_minutes(new Date(created_at)),
        },
      ]}
    />
  );
};

export const ServerComponents: RequiredResourceComponents = {
  Name: ({ id }: { id: string }) => <>{useServer(id)?.name}</>,
  Description: ({ id }) => <>{useServer(id)?.info.status}</>,
  Info: ({ id }) => <ServerInfo id={id} />,
  Actions: () => null,
  Icon: ServerIconComponent,
  Page: {
    Stats: ({ id }) => <ServerStats server_id={id} />,
    Deployments: ({ id }) => {
      const deployments = useRead("ListDeployments", {}).data?.filter(deployment => deployment.info.server_id === id);
      return (
        <Section title="Deployments" icon={<Rocket className="w-4 h-4" />}>
          <DeploymentTable deployments={deployments} />
        </Section>
      );
    },
    Config: ({ id }) => <ServerConfig id={id} />,
    Danger: ({ id }) => (
      <Section title="Danger Zone" icon={<AlertTriangle className="w-4 h-4" />}>
        <RenameServer id={id} />
        <DeleteServer id={id} />
      </Section>
    ),
  },
  New: () => <NewServer />,
  Table: ServerTable,
  Dashboard: ServersChart,
};
