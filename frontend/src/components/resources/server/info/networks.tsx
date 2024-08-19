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
            accessorKey: "Name",
            header: ({ column }) => (
              <SortableHeader column={column} title="Name" />
            ),
            cell: ({ row }) => (
              <div>
                <Link to={`/servers/${id}/network/${row.original.Name}`}>
                  <Button variant="link">{row.original.Name}</Button>
                </Link>
                {["none", "host", "bridge"].includes(
                  row.original.Name ?? ""
                ) && <Badge variant="outline">System</Badge>}
              </div>
            ),
          },
          {
            accessorKey: "Driver",
            header: ({ column }) => (
              <SortableHeader column={column} title="Driver" />
            ),
          },
          {
            accessorKey: "Attachable",
            header: ({ column }) => (
              <SortableHeader column={column} title="Attachable" />
            ),
          },
          {
            accessorKey: "IPAM.Driver",
            header: ({ column }) => (
              <SortableHeader column={column} title="IPAM Driver" />
            ),
          },
        ]}
      />
    </Section>
  );
};
