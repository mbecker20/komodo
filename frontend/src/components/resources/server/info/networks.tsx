import { Section } from "@components/layouts";
import { DockerResourceLink } from "@components/util";
import { useRead } from "@lib/hooks";
import { Badge } from "@ui/badge";
import { DataTable, SortableHeader } from "@ui/data-table";
import { ReactNode } from "react";

export const Networks = ({
  id,
  titleOther,
}: {
  id: string;
  titleOther: ReactNode;
}) => {
  const networks =
    useRead("ListDockerNetworks", { server: id }, { refetchInterval: 10_000 })
      .data ?? [];

  // const allInUse = networks.every((network) =>
  //   // this ignores networks that come in with no name, but they should all come in with name
  //   !network.name
  //     ? true
  //     : ["none", "host", "bridge"].includes(network.name)
  //       ? true
  //       : network.in_use
  // );

  return (
    <Section titleOther={titleOther}>
      <DataTable
        tableKey="server-networks"
        data={networks}
        columns={[
          {
            accessorKey: "name",
            header: ({ column }) => (
              <SortableHeader column={column} title="Name" />
            ),
            cell: ({ row }) => (
              <div className="flex items-center gap-2">
                <DockerResourceLink
                  type="network"
                  server_id={id}
                  name={row.original.name}
                  extra={
                    ["none", "host", "bridge"].includes(
                      row.original.name ?? ""
                    ) ? (
                      <Badge variant="outline">System</Badge>
                    ) : (
                      !row.original.in_use && (
                        <Badge variant="destructive">Unused</Badge>
                      )
                    )
                  }
                />
              </div>
            ),
            size: 300,
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
          {
            accessorKey: "attachable",
            header: ({ column }) => (
              <SortableHeader column={column} title="Attachable" />
            ),
          },
          {
            accessorKey: "ipam_driver",
            header: ({ column }) => (
              <SortableHeader column={column} title="IPAM Driver" />
            ),
          },
        ]}
      />
    </Section>
  );
};
