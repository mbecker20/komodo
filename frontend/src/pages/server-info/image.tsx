import { Section } from "@components/layouts";
import { ResourceLink } from "@components/resources/common";
import { useServer } from "@components/resources/server";
import { DockerLabelsSection } from "@components/util";
import { format_size_bytes } from "@lib/formatting";
import { useRead, useSetTitle } from "@lib/hooks";
import { Button } from "@ui/button";
import { DataTable } from "@ui/data-table";
import { ChevronLeft, HardDrive, Info, Loader2 } from "lucide-react";
import { useNavigate, useParams } from "react-router-dom";

export const ImagePage = () => {
  const { type, id, image } = useParams() as {
    type: string;
    id: string;
    image: string;
  };
  if (type !== "servers") {
    return <div>This resource type does not have any images.</div>;
  }
  return <ImagePageInner id={id} image={decodeURIComponent(image)} />;
};

const ImagePageInner = ({
  id,
  image: image_name,
}: {
  id: string;
  image: string;
}) => {
  const server = useServer(id);
  useSetTitle(`${server?.name} | image | ${image_name}`);
  const nav = useNavigate();
  // const perms = useRead("GetPermissionLevel", {
  //   target: { type: "Server", id },
  // }).data;
  // const list_image = useRead("ListDockerImages", { server: id }).data?.find(
  //   (image) => image.name === _image
  // );
  const {
    data: image,
    isPending,
    isError,
  } = useRead("InspectDockerImage", {
    server: id,
    image: image_name,
  });
  // const history = useRead("ListDockerImageHistory", {
  //   server: id,
  //   image: _image,
  // }).data;

  if (isPending) {
    return (
      <div className="flex justify-center w-full py-4">
        <Loader2 className="w-8 h-8 animate-spin" />
      </div>
    );
  }
  if (isError) {
    return <div className="flex w-full py-4">Failed to inspect image.</div>;
  }
  if (!image) {
    return (
      <div className="flex w-full py-4">
        No image found with given name: {image_name}
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
        <div className="flex items-center gap-4">
          <div className="mt-1">
            <HardDrive className="w-8 h-8" />
          </div>
          <h1
            title={image_name}
            className="text-3xl max-w-[60vw] overflow-hidden text-ellipsis"
          >
            {image_name}
          </h1>
        </div>

        {/* INFO */}
        <div className="flex flex-wrap gap-4 items-center text-muted-foreground">
          <ResourceLink type="Server" id={id} />
          {image.Id ? (
            <>
              |
              <div className="flex gap-2">
                Id:
                <div
                  title={image.Id}
                  className="max-w-[150px] overflow-hidden text-ellipsis"
                >
                  {image.Id}
                </div>
              </div>
            </>
          ) : null}
        </div>
      </div>

      {/* TOP LEVEL IMAGE INFO */}
      <Section title="Details" icon={<Info className="w-4 h-4" />}>
        <DataTable
          tableKey="image-info"
          data={[image]}
          columns={[
            {
              accessorKey: "Architecture",
              header: "Architecture",
            },
            {
              accessorKey: "Os",
              header: "Os",
            },
            {
              accessorKey: "Size",
              header: "Size",
              cell: ({ row }) =>
                row.original.Size
                  ? format_size_bytes(row.original.Size)
                  : "Unknown",
            },
          ]}
        />
      </Section>

      {/* {history && history.length > 0 && <Section title="History"></Section>} */}

      <DockerLabelsSection labels={image?.Config?.Labels} />
    </div>
  );
};
