import { DataTable, SortableHeader } from "@ui/data-table";
import { ResourceLink } from "../common";
import { TableTags } from "@components/tags";
import { Types } from "@monitor/client";

export const AlerterTable = ({
  alerters,
}: {
  alerters: Types.AlerterListItem[];
}) => {
  return (
    <DataTable
      tableKey="alerters"
      data={alerters}
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
