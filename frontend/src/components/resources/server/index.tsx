import { useRead, useWrite } from "@lib/hooks";
import { cn } from "@lib/utils";
import { Types } from "@monitor/client";
import { RequiredResourceComponents } from "@types";
import { ServerIcon, AlertTriangle, Rocket, LineChart } from "lucide-react";
import { useState } from "react";
import { NewResource, Section } from "@components/layouts";
import { Input } from "@ui/input";
import { RenameServer, SERVER_ACTIONS } from "./actions";
import {
  fill_color_class_by_intention,
  server_status_intention,
  text_color_class_by_intention,
} from "@lib/color";
import { ServerConfig } from "./config";
import { DeploymentTable } from "../deployment/table";
import { ServerTable } from "./table";
import { ServersChart } from "./dashboard";
import { ResourceLink } from "@components/util";
import { Link } from "react-router-dom";
import { Button } from "@ui/button";
import { DeleteResource } from "@components/config/util";

export const useServer = (id?: string) =>
  useRead("ListServers", {}).data?.find((d) => d.id === id);

export const ServerComponents: RequiredResourceComponents = {
  Name: ({ id }: { id: string }) => <>{useServer(id)?.name}</>,
  Description: ({ id }) => <>{useServer(id)?.info.status}</>,
  Link: ({ id }) => <ResourceLink type="Server" id={id} />,
  Info: [
    ({ id }) => (
      <Link to={`/servers/${id}/stats`}>
        <Button variant="link" className="flex gap-2 items-center p-0">
          <LineChart className="w-4 h-4" />
          Stats
        </Button>
      </Link>
    ),
  ],
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
  Actions: SERVER_ACTIONS,
  Page: {
    // Stats: ({ id }) => {
    //   const status = useServer(id)?.info.status;
    //   return status === "Ok" && <ServerStats server_id={id} />;
    // },
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
        <DeleteResource type="Server" id={id} />
      </Section>
    ),
  },
  New: () => {
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
  },
  Table: ServerTable,
  Dashboard: ServersChart,
};
