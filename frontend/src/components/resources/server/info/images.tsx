import { Section } from "@components/layouts";
import { DockerResourceLink } from "@components/util";
import { format_size_bytes } from "@lib/formatting";
import { useRead } from "@lib/hooks";
import { Badge } from "@ui/badge";
import { DataTable, SortableHeader } from "@ui/data-table";
import { ReactNode } from "react";
import { Prune } from "../actions";

export const Images = ({
  id,
  titleOther,
}: {
  id: string;
  titleOther: ReactNode;
}) => {
  const images =
    useRead("ListDockerImages", { server: id }, { refetchInterval: 10_000 })
      .data ?? [];

  const allInUse = images.every((image) => image.in_use);

  return (
    <Section
      titleOther={titleOther}
      actions={!allInUse && <Prune server_id={id} type="Images" />}
    >
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
                  !row.original.in_use && (
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
    </Section>
  );
};
