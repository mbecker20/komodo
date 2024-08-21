import { Section } from "@components/layouts";
import { ResourceLink } from "@components/resources/common";
import { useServer } from "@components/resources/server";
import { DockerLabelsSection } from "@components/util";
import { useRead, useSetTitle } from "@lib/hooks";
import { Button } from "@ui/button";
import { DataTable } from "@ui/data-table";
import { ChevronLeft, Database, Info, Loader2 } from "lucide-react";
import { useNavigate, useParams } from "react-router-dom";

export const VolumePage = () => {
  const { type, id, volume } = useParams() as {
    type: string;
    id: string;
    volume: string;
  };
  if (type !== "servers") {
    return <div>This resource type does not have any volumes.</div>;
  }
  return <VolumePageInner id={id} volume={decodeURIComponent(volume)} />;
};

const VolumePageInner = ({
  id,
  volume: _volume,
}: {
  id: string;
  volume: string;
}) => {
  const server = useServer(id);
  useSetTitle(`${server?.name} | volume | ${_volume}`);
  const nav = useNavigate();
  // const perms = useRead("GetPermissionLevel", {
  //   target: { type: "Server", id },
  // }).data;
  const {
    data: volume,
    isPending,
    isError,
  } = useRead("InspectDockerVolume", {
    server: id,
    volume: _volume,
  });

  if (isPending) {
    return (
      <div className="flex justify-center w-full py-4">
        <Loader2 className="w-8 h-8 animate-spin" />
      </div>
    );
  }
  if (isError) {
    return (
      <div className="flex w-full py-4">
        Failed to get volume list for server.
      </div>
    );
  }
  if (!volume) {
    return (
      <div className="flex w-full py-4">
        No volume found with given name: {_volume}
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
              <Database className="w-8 h-8" />
            </div>
            <h1 className="text-3xl">{volume.Name}</h1>
          </div>
        </div>

        {/* INFO */}
        <div className="flex flex-wrap gap-4 items-center text-muted-foreground">
          <ResourceLink type="Server" id={id} />
        </div>
      </div>

      {/* TOP LEVEL VOLUME INFO */}
      <Section title="Details" icon={<Info className="w-4 h-4" />}>
        <DataTable
          tableKey="image-info"
          data={[volume]}
          columns={[
            {
              accessorKey: "",
              header: "Driver",
            },
            {
              accessorKey: "Scope",
              header: "Scope",
            },
            {
              accessorKey: "Attachable",
              header: "Attachable",
            },
            {
              accessorKey: "Internal",
              header: "Internal",
            },
          ]}
        />
      </Section>

      <DockerLabelsSection labels={volume.Labels} />
    </div>
  );
};
