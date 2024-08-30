import { Section } from "@components/layouts";
import { DockerResourceLink, ShowHideButton } from "@components/util";
import { format_size_bytes } from "@lib/formatting";
import { useRead } from "@lib/hooks";
import { Badge } from "@ui/badge";
import { DataTable, SortableHeader } from "@ui/data-table";
import { HardDrive } from "lucide-react";
import { useCallback, useMemo } from "react";
import { Prune } from "../actions";

export const Images = ({
  id,
  show,
  setShow,
}: {
  id: string;
  show: boolean;
  setShow: (show: boolean) => void;
}) => {
  const images =
    useRead("ListDockerImages", { server: id }, { refetchInterval: 5000 })
      .data ?? [];
  const containers = useRead("ListDockerContainers", { server: id }).data ?? [];

  const no_containers = useCallback(
    (image_id: string) => {
      return containers.every((container) => container.image_id !== image_id);
    },
    [containers]
  );

  console.log(images);

  const allInUse = useMemo(() => {
    return images.every((image) =>
      // this ignores images that come in with no id, but they should all come in with id
      !image.id ? true : !no_containers(image.id)
    );
  }, [no_containers]);

  return (
    <div className={show ? "mb-8" : undefined}>
      <Section
        title="Images"
        icon={<HardDrive className="w-4 h-4" />}
        actions={
          <div className="flex items-center gap-2">
            {!allInUse && <Prune server_id={id} type="Images" />}
            <ShowHideButton show={show} setShow={setShow} />
          </div>
        }
      >
        {show && (
          <DataTable
            tableKey="server-images"
            data={images}
            columns={[
              {
                accessorKey: "name",
                header: ({ column }) => (
                  <SortableHeader column={column} title="Name" />
                ),
                cell: ({ row }) => (
                  <DockerResourceLink
                    type="image"
                    server_id={id}
                    name={row.original.name}
                    id={row.original.id}
                    extra={
                      row.original.id &&
                      no_containers(row.original.id) && (
                        <Badge variant="destructive">Unused</Badge>
                      )
                    }
                  />
                ),
                size: 200,
              },
              {
                accessorKey: "id",
                header: ({ column }) => (
                  <SortableHeader column={column} title="Id" />
                ),
              },
              {
                accessorKey: "size",
                header: ({ column }) => (
                  <SortableHeader column={column} title="Size" />
                ),
                cell: ({ row }) =>
                  row.original.size
                    ? format_size_bytes(row.original.size)
                    : "Unknown",
              },
            ]}
          />
        )}
      </Section>
    </div>
  );
};
