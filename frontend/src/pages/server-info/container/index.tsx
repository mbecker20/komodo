import { Section } from "@components/layouts";
import { NewResource, ResourceLink } from "@components/resources/common";
import { useServer } from "@components/resources/server";
import {
  DockerLabelsSection,
  DockerResourcePageName,
  StatusBadge,
} from "@components/util";
import { useRead, useSetTitle } from "@lib/hooks";
import { Button } from "@ui/button";
import { DataTable } from "@ui/data-table";
import { Box, ChevronLeft, Clapperboard, Info, Loader2 } from "lucide-react";
import { useNavigate, useParams } from "react-router-dom";
import { ContainerLogs } from "./log";
import { Actions } from "./actions";
import { has_minimum_permissions } from "@lib/utils";
import { Types } from "@monitor/client";
import { ResourceUpdates } from "@components/updates/resource";
import { container_state_intention } from "@lib/color";
import { UsableResource } from "@types";

export const ContainerPage = () => {
  const { type, id, container } = useParams() as {
    type: string;
    id: string;
    container: string;
  };
  if (type !== "servers") {
    return <div>This resource type does not have any containers.</div>;
  }
  return (
    <ContainerPageInner id={id} container={decodeURIComponent(container)} />
  );
};

const ContainerPageInner = ({
  id,
  container: container_name,
}: {
  id: string;
  container: string;
}) => {
  const server = useServer(id);
  useSetTitle(`${server?.name} | container | ${container_name}`);
  const nav = useNavigate();
  const perms = useRead("GetPermissionLevel", {
    target: { type: "Server", id },
  }).data;
  const {
    data: container,
    isPending,
    isError,
  } = useRead("InspectDockerContainer", {
    server: id,
    container: container_name,
  });
  const list_container = useRead(
    "ListDockerContainers",
    {
      server: id,
    },
    { refetchInterval: 30_000 }
  ).data?.find((container) => container.name === container_name);

  if (isPending) {
    return (
      <div className="flex justify-center w-full py-4">
        <Loader2 className="w-8 h-8 animate-spin" />
      </div>
    );
  }
  if (isError) {
    return <div className="flex w-full py-4">Failed to inspect container.</div>;
  }
  if (!container) {
    return (
      <div className="flex w-full py-4">
        No container found with given name: {container_name}
      </div>
    );
  }

  const canExecute = has_minimum_permissions(
    perms,
    Types.PermissionLevel.Execute
  );

  const state = list_container?.state ?? Types.ContainerStateStatusEnum.Empty;
  const status = list_container?.status;

  return (
    <div className="flex flex-col gap-16 mb-24">
      {/* HEADER */}
      <div className="flex flex-col gap-4">
        {/* BACK */}
        <div className="flex items-center justify-between mb-4">
          <Button
            className="gap-2"
            variant="secondary"
            onClick={() => nav("/servers/" + id)}
          >
            <ChevronLeft className="w-4" /> Back
          </Button>

          <NewDeployment id={id} container={container_name} />
        </div>

        {/* TITLE */}
        <div className="flex items-center gap-4">
          <div className="mt-1">
            <Box className="w-8 h-8" />
          </div>
          <DockerResourcePageName name={container_name} />
          <div className="flex items-center gap-4 flex-wrap">
            <StatusBadge
              text={state}
              intent={container_state_intention(state)}
            />
            {status && (
              <p className="text-sm text-muted-foreground">{status}</p>
            )}
          </div>
        </div>

        {/* INFO */}
        <div className="flex flex-wrap gap-4 items-center text-muted-foreground">
          <ResourceLink type="Server" id={id} />
          <AttachedResource id={id} container={container_name} />
        </div>
      </div>

      {/* Actions */}
      {canExecute && (
        <Section title="Actions" icon={<Clapperboard className="w-4 h-4" />}>
          <div className="flex gap-4 items-center flex-wrap">
            {Object.entries(Actions).map(([key, Action]) => (
              <Action key={key} id={id} container={container_name} />
            ))}
          </div>
        </Section>
      )}

      {/* Updates */}
      <ResourceUpdates type="Server" id={id} />

      <ContainerLogs id={id} container_name={container_name} />

      {/* TOP LEVEL CONTAINER INFO */}
      <Section title="Details" icon={<Info className="w-4 h-4" />}>
        <DataTable
          tableKey="container-info"
          data={[container]}
          columns={[
            {
              accessorKey: "Image",
              header: "Image",
            },
            {
              accessorKey: "Driver",
              header: "Driver",
            },
          ]}
        />
      </Section>

      <DockerLabelsSection labels={container.Config?.Labels} />
    </div>
  );
};

const AttachedResource = ({
  id,
  container,
}: {
  id: string;
  container: string;
}) => {
  const { data: attached, isPending } = useRead(
    "GetResourceMatchingContainer",
    { server: id, container },
    { refetchInterval: 30_000 }
  );

  if (isPending) {
    return <Loader2 className="w-4 h-4 animate-spin" />;
  }

  if (!attached || !attached.resource) {
    return null;
  }

  return (
    <>
      |
      <div className="flex gap-2">
        <div>{attached.resource.type}:</div>
        <ResourceLink
          type={attached.resource.type as UsableResource}
          id={attached.resource.id}
        />
      </div>
    </>
  );
};

const NewDeployment = ({
  id,
  container,
}: {
  id: string;
  container: string;
}) => {
  const { data: attached, isPending } = useRead(
    "GetResourceMatchingContainer",
    { server: id, container }
  );

  if (isPending) {
    return <Loader2 className="w-4 h-4 animate-spin" />;
  }

  if (!attached) {
    return null;
  }

  if (!attached?.resource) {
    return <NewResource type="Deployment" server_id={id} name={container} />;
  }
};
