import { Section } from "@components/layouts";
import { useRead } from "@lib/hooks";
import { Badge } from "@ui/badge";
import { Button } from "@ui/button";
import { DataTable, SortableHeader } from "@ui/data-table";
import { Link } from "react-router-dom";

export const Networks = ({ id }: { id: string }) => {
  const networks = useRead("ListDockerNetworks", { server: id }).data ?? [];

  return (
    <Section title="Networks">
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
              <div>
                <Link to={`/servers/${id}/network/${row.original.name}`}>
                  <Button variant="link">{row.original.name}</Button>
                </Link>
                {["none", "host", "bridge"].includes(
                  row.original.name ?? ""
                ) && <Badge variant="outline">System</Badge>}
              </div>
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
