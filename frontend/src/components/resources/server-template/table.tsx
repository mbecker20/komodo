import { useRead, useTagsFilter } from "@lib/hooks";
import { DataTable, SortableHeader } from "@ui/data-table";
import { ResourceLink } from "../common";
import { TagsWithBadge } from "@components/tags";

export const ServerTemplateTable = ({ search }: { search?: string }) => {
  const tags = useTagsFilter();
  const server_templates = useRead("ListServerTemplates", {}).data;
  const searchSplit = search?.split(" ") || [];
  return (
    <DataTable
      tableKey="server-templates"
      data={
        server_templates?.filter(
          (resource) =>
            tags.every((tag) => resource.tags.includes(tag)) &&
            (searchSplit.length > 0
              ? searchSplit.every((search) => resource.name.includes(search))
              : true)
        ) ?? []
      }
      columns={[
        {
          accessorKey: "name",
          header: ({ column }) => (
            <SortableHeader column={column} title="Name" />
          ),
          cell: ({ row }) => (
            <ResourceLink type="ServerTemplate" id={row.original.id} />
          ),
        },
        {
          accessorKey: "info.provider",
          header: ({ column }) => (
            <SortableHeader column={column} title="Provider" />
          ),
        },
        {
          accessorKey: "info.instance_type",
          header: ({ column }) => (
            <SortableHeader column={column} title="Instance Type" />
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
