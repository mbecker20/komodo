import { useRead } from "@lib/hooks";
import { DataTable } from "@ui/data-table";
import { Link } from "react-router-dom";
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
          cell: ({ row }) => {
            const id = row.original.id;
            return (
              <Link
                to={`/procedures/${id}`}
                className="flex items-center gap-2"
              >
                <ProcedureComponents.Icon id={id} />
                <ProcedureComponents.Name id={id} />
              </Link>
            );
          },
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