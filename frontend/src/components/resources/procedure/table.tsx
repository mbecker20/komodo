import { useRead } from "@lib/hooks";
import { DataTable } from "@ui/data-table";
import { ProcedureComponents } from ".";
import { TagsWithBadge, useTagsFilter } from "@components/tags";

export const ProcedureTable = ({ search }: { search: string | undefined }) => {
  const tags = useTagsFilter();
  const procedures = useRead("ListProcedures", {}).data;
  const searchSplit = search?.split(" ") || [];
  return (
    <DataTable
      data={
        procedures?.filter((resource) =>
          tags.every((tag) => resource.tags.includes(tag)) &&
          searchSplit.length > 0
            ? searchSplit.every((search) => resource.name.includes(search))
            : true
        ) ?? []
      }
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
