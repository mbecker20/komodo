import { useFilterResources, useRead } from "@lib/hooks";
import { DataTable, SortableHeader } from "@ui/data-table";
import { TagsWithBadge } from "@components/tags";
import { ResourceLink } from "../common";
import { ProcedureComponents } from ".";

export const ProcedureTable = ({ search }: { search?: string }) => {
  const procedures = useRead("ListProcedures", {}).data;
  const filtered = useFilterResources(procedures, search);
  return (
    <DataTable
      tableKey="procedures"
      data={filtered}
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
          cell: ({ row }) => (
            <ProcedureComponents.Status.State id={row.original.id} />
          ),
        },
        {
          header: "Tags",
          cell: ({ row }) => {
            return (
              <div className="flex gap-1">
                <TagsWithBadge tag_ids={row.original.tags} />
              </div>
            );
          },
        },
      ]}
    />
  );
};
