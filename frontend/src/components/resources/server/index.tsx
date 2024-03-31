import { useRead, useWrite } from "@lib/hooks";
import { cn } from "@lib/utils";
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
import {
  fill_color_class_by_intention,
  server_status_intention,
  text_color_class_by_intention,
} from "@lib/color";
import { ServerConfig } from "./config";

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

export const ServerIconComponent = ({ id }: { id?: string }) => {
  const status = useServer(id)?.info.status;
  return (
    <ServerIcon
      className={cn(
        "w-4 h-4",
        id && fill_color_class_by_intention(server_status_intention(status))
      )}
    />
  );
};

const NewServer = () => {
  const { mutateAsync } = useWrite("CreateServer");
  const [name, setName] = useState("");
  return (
    <NewResource
      entityType="Server"
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
          header: "State",
          cell: ({
            row: {
              original: { id },
            },
          }) => <ServerComponents.Status id={id} />,
        },
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
  Status: ({ id }) => {
    const status = useServer(id)?.info.status;
    const stateClass = text_color_class_by_intention(
      server_status_intention(status)
    );
    return (
      <div className={stateClass}>
        {status === Types.ServerStatus.NotOk ? "Not Ok" : status}
      </div>
    );
  },
  Page: {
    Stats: ({ id }) => <ServerStats server_id={id} />,
    Deployments: ({ id }) => {
      const deployments = useRead("ListDeployments", {}).data?.filter(
        (deployment) => deployment.info.server_id === id
      );
      return (
        <Section title="Deployments" icon={<Rocket className="w-4 h-4" />}>
          <DeploymentTable deployments={deployments} />
        </Section>
      );
    },
    Config: ServerConfig,
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
