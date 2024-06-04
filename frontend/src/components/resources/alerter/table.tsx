import { useFilterResources, useRead } from "@lib/hooks";
import { DataTable, SortableHeader } from "@ui/data-table";
import { ResourceLink } from "../common";
import { TableTags } from "@components/tags";

export const AlerterTable = ({ search }: { search?: string }) => {
  const alerters = useRead("ListAlerters", {}).data;
  const filtered = useFilterResources(alerters, search);
  return (
    <DataTable
      tableKey="alerters"
      data={filtered}
      columns={[
        {
          accessorKey: "name",
          header: ({ column }) => (
            <SortableHeader column={column} title="Name" />
          ),
          cell: ({ row }) => (
            <ResourceLink type="Alerter" id={row.original.id} />
          ),
        },
        {
          accessorKey: "info.alerter_type",
          header: ({ column }) => (
            <SortableHeader column={column} title="Type" />
          ),
        },
        {
          header: "Tags",
          cell: ({ row }) => <TableTags tag_ids={row.original.tags} />,
        },
      ]}
    />
  );
};
