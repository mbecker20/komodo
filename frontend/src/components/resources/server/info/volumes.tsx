import { Section } from "@components/layouts";
import { DockerResourceLink } from "@components/util";
import { useRead } from "@lib/hooks";
import { Badge } from "@ui/badge";
import { DataTable, SortableHeader } from "@ui/data-table";
import { ReactNode } from "react";

export const Volumes = ({
  id,
  titleOther,
}: {
  id: string;
  titleOther: ReactNode;
}) => {
  const volumes =
    useRead("ListDockerVolumes", { server: id }, { refetchInterval: 10_000 })
      .data ?? [];

  return (
    <Section titleOther={titleOther}>
      <DataTable
        tableKey="server-volumes"
        data={volumes}
        columns={[
          {
            accessorKey: "name",
            header: ({ column }) => (
              <SortableHeader column={column} title="Name" />
            ),
            cell: ({ row }) => (
              <DockerResourceLink
                type="volume"
                server_id={id}
                name={row.original.name}
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
            accessorKey: "driver",
            header: ({ column }) => (
              <SortableHeader column={column} title="Driver" />
            ),
          },
          {
            accessorKey: "scope",
            header: ({ column }) => (
              <SortableHeader column={column} title="Scope" />
            ),
          },
        ]}
      />
    </Section>
  );
};
