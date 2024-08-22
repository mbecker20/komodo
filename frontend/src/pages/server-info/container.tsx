import { Section } from "@components/layouts";
import { ResourceLink } from "@components/resources/common";
import { useServer } from "@components/resources/server";
import { DockerLabelsSection } from "@components/util";
import { useRead, useSetTitle } from "@lib/hooks";
import { Button } from "@ui/button";
import { DataTable } from "@ui/data-table";
import { Box, ChevronLeft, Info, Loader2 } from "lucide-react";
import { useNavigate, useParams } from "react-router-dom";

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
  container: _container,
}: {
  id: string;
  container: string;
}) => {
  const server = useServer(id);
  useSetTitle(`${server?.name} | container | ${_container}`);
  const nav = useNavigate();
  // const perms = useRead("GetPermissionLevel", {
  //   target: { type: "Server", id },
  // }).data;
  const {
    data: container,
    isPending,
    isError,
  } = useRead("InspectDockerContainer", {
    server: id,
    container: _container,
  });

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
        No container found with given name: {_container}
      </div>
    );
  }

  // const disabled = !has_minimum_permissions(perms, Types.PermissionLevel.Write);

  return (
    <div className="flex flex-col gap-16">
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

          {/* <Button className="gap-2" variant="destructive">
            <Trash className="w-4" /> Delete
          </Button> */}
        </div>

        {/* TITLE */}
        <div className="flex flex-col gap-4">
          <div className="flex items-center gap-4">
            <div className="mt-1">
              <Box className="w-8 h-8" />
            </div>
            <h1 className="text-3xl">{container.Name}</h1>
          </div>
        </div>

        {/* INFO */}
        <div className="flex flex-wrap gap-4 items-center text-muted-foreground">
          <ResourceLink type="Server" id={id} />
        </div>
      </div>

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
