import { Section } from "@components/layouts";
import { DockerResourceLink, ShowHideButton } from "@components/util";
import { format_size_bytes } from "@lib/formatting";
import { useRead } from "@lib/hooks";
import { DataTable, SortableHeader } from "@ui/data-table";
import { HardDrive } from "lucide-react";

export const Images = ({
  id,
  show,
  setShow,
}: {
  id: string;
  show: boolean;
  setShow: (show: boolean) => void;
}) => {
  const images = useRead("ListDockerImages", { server: id }).data ?? [];

  return (
    <Section
      title="Images"
      icon={<HardDrive className="w-4 h-4" />}
      actions={<ShowHideButton show={show} setShow={setShow} />}
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
  );
};
