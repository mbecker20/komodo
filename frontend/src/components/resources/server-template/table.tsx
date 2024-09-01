import { DataTable, SortableHeader } from "@ui/data-table";
import { ResourceLink } from "../common";
import { TableTags } from "@components/tags";
import { Types } from "@komodo/client";

export const ServerTemplateTable = ({
  serverTemplates,
}: {
  serverTemplates: Types.ServerTemplateListItem[];
}) => {
  return (
    <DataTable
      tableKey="server-templates"
      data={serverTemplates}
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
