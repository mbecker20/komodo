import { useRead } from "@lib/hooks";
import { DataTable } from "@ui/data-table";
import { ProcedureComponents } from ".";
import { TagsWithBadge } from "@components/tags";

export const ProcedureTable = () => {
  const procedures = useRead("ListProcedures", {}).data;
  return (
    <DataTable
      data={procedures ?? []}
      columns={[
        {
          accessorKey: "id",
          header: "Name",
          cell: ({ row }) => <ProcedureComponents.Link id={row.original.id} />,
        },
        {
          header: "Type",
          accessorKey: "info.procedure_type",
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
