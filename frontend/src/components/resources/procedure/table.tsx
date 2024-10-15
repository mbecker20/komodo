import { DataTable, SortableHeader } from "@ui/data-table";
import { TableTags } from "@components/tags";
import { ResourceLink } from "../common";
import { ProcedureComponents } from ".";
import { Types } from "komodo_client";

export const ProcedureTable = ({
  procedures,
}: {
  procedures: Types.ProcedureListItem[];
}) => {
  return (
    <DataTable
      tableKey="procedures"
      data={procedures}
      columns={[
        {
          accessorKey: "name",
          header: ({ column }) => (
            <SortableHeader column={column} title="Name" />
          ),
          cell: ({ row }) => (
            <ResourceLink type="Procedure" id={row.original.id} />
          ),
        },
        {
          accessorKey: "info.stages",
          header: ({ column }) => (
            <SortableHeader column={column} title="Stages" />
          ),
        },
        {
          accessorKey: "info.state",
          header: ({ column }) => (
            <SortableHeader column={column} title="State" />
          ),
          cell: ({ row }) => <ProcedureComponents.State id={row.original.id} />,
        },
        {
          header: "Tags",
          cell: ({ row }) => <TableTags tag_ids={row.original.tags} />,
        },
      ]}
    />
  );
};
