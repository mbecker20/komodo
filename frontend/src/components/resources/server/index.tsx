import { useExecute, useRead } from "@lib/hooks";
import { cn } from "@lib/utils";
import { Types } from "@monitor/client";
import { RequiredResourceComponents } from "@types";
import {
  ServerIcon,
  AlertTriangle,
  Rocket,
  Cpu,
  MemoryStick,
  Database,
  ExternalLink,
  Scissors,
  XOctagon,
} from "lucide-react";
import { Section } from "@components/layouts";
import { RenameServer } from "./actions";
import {
  fill_color_class_by_intention,
  server_status_intention,
  text_color_class_by_intention,
} from "@lib/color";
import { ServerConfig } from "./config";
import { DeploymentTable } from "../deployment/table";
import { ServerTable } from "./table";
import { ServersChart } from "./dashboard";
import { Link } from "react-router-dom";
import { DeleteResource, NewResource } from "../common";
import { AlertsTable } from "@components/alert/table";
import { Button } from "@ui/button";
import { ActionWithDialog, ConfirmButton } from "@components/util";

export const useServer = (id?: string) =>
  useRead("ListServers", {}).data?.find((d) => d.id === id);

export const ServerComponents: RequiredResourceComponents = {
  Dashboard: ServersChart,

  New: () => <NewResource type="Server" />,

  Table: ServerTable,

  Name: ({ id }: { id: string }) => <>{useServer(id)?.name}</>,

  Icon: ({ id }) => {
    const status = useServer(id)?.info.status;
    return (
      <ServerIcon
        className={cn(
          "w-4 h-4",
          id && fill_color_class_by_intention(server_status_intention(status))
        )}
      />
    );
  },

  Status: [
    ({ id }) => {
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
  ],

  Info: [
    ({ id }) => {
      const server = useServer(id);
      const core_count =
        useRead(
          "GetSystemInformation",
          { server: id },
          { enabled: server ? server.info.status !== "Disabled" : false }
        ).data?.core_count ?? 0;
      return (
        <Link to={`/servers/${id}/stats`} className="flex gap-2 items-center">
          <Cpu className="w-4 h-4" />
          {core_count || "N/A"} Core{core_count > 1 ? "s" : ""}
        </Link>
      );
    },
    ({ id }) => {
      const server = useServer(id);
      const stats = useRead(
        "GetSystemStats",
        { server: id },
        { enabled: server ? server.info.status !== "Disabled" : false }
      ).data;
      return (
        <Link to={`/servers/${id}/stats`} className="flex gap-2 items-center">
          <MemoryStick className="w-4 h-4" />
          {stats?.mem_total_gb.toFixed(2) ?? "N/A"} GB
        </Link>
      );
    },
    ({ id }) => {
      const server = useServer(id);
      const stats = useRead(
        "GetSystemStats",
        { server: id },
        { enabled: server ? server.info.status !== "Disabled" : false }
      ).data;
      const disk_total_gb = stats?.disks.reduce(
        (acc, curr) => acc + curr.total_gb,
        0
      );
      return (
        <Link to={`/servers/${id}/stats`} className="flex gap-2 items-center">
          <Database className="w-4 h-4" />
          {disk_total_gb?.toFixed(2) ?? "N/A"} GB
        </Link>
      );
    },
  ],

  Actions: [
    ({ id }) => {
      const { mutate, isPending } = useExecute(`PruneImages`);
      const pruning = useRead("GetServerActionState", { server: id }).data
        ?.pruning_images;
      const pending = isPending || pruning;
      return (
        <ConfirmButton
          title="Prune Images"
          icon={<Scissors className="w-4 h-4" />}
          onClick={() => mutate({ server: id })}
          loading={pending}
          disabled={pending}
        />
      );
    },
    ({ id }) => {
      const server = useServer(id);
      const { mutate, isPending } = useExecute(`StopAllContainers`);
      const stopping = useRead("GetServerActionState", { server: id }).data
        ?.stopping_containers;
      const pending = isPending || stopping;
      return (
        server && (
          <ActionWithDialog
            name={server?.name}
            title="Stop Containers"
            icon={<XOctagon className="w-4 h-4" />}
            onClick={() => mutate({ server: id })}
            disabled={pending}
            loading={pending}
          />
        )
      );
    },
  ],

  Page: {
    Alerts: ({ id }) => {
      const alerts = useRead("ListAlerts", {
        query: { "target.type": "Server", "target.id": id },
      }).data?.alerts.slice(0, 3);
      return (
        (alerts?.length || 0) > 0 && (
          <Section
            title="Alerts"
            icon={<AlertTriangle className="w-4 h-4" />}
            actions={
              <Link to={`/servers/${id}/alerts`}>
                <Button variant="secondary" size="icon">
                  <ExternalLink className="w-4 h-4" />
                </Button>
              </Link>
            }
          >
            <AlertsTable alerts={alerts ?? []} />
          </Section>
        )
      );
    },
    Deployments: ({ id }) => {
      const deployments = useRead("ListDeployments", {}).data?.filter(
        (deployment) => deployment.info.server_id === id
      );
      return (
        (deployments?.length || 0) > 0 && (
          <Section title="Deployments" icon={<Rocket className="w-4 h-4" />}>
            <DeploymentTable deployments={deployments} />
          </Section>
        )
      );
    },
  },

  Config: ServerConfig,

  DangerZone: ({ id }) => (
    <>
      <RenameServer id={id} />
      <DeleteResource type="Server" id={id} />
    </>
  ),
};
