import { useFilterResources, useRead } from "@lib/hooks";
import { DataTable, SortableHeader } from "@ui/data-table";
import { ResourceLink } from "../common";
import { TableTags } from "@components/tags";

export const ServerTemplateTable = ({ search }: { search?: string }) => {
  const server_templates = useRead("ListServerTemplates", {}).data;
  const filtered = useFilterResources(server_templates, search);
  return (
    <DataTable
      tableKey="server-templates"
      data={filtered}
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
          cell: ({ row }) => <TableTags tag_ids={row.original.tags} />,
        },
      ]}
    />
  );
};
