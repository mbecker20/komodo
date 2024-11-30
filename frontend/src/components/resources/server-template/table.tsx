import { DataTable, SortableHeader } from "@ui/data-table";
import { ResourceLink } from "../common";
import { TableTags } from "@components/tags";
import { Types } from "komodo_client";
import { useSelectedResources } from "@lib/hooks";

export const ServerTemplateTable = ({
  serverTemplates,
}: {
  serverTemplates: Types.ServerTemplateListItem[];
}) => {
  const [_, setSelectedResources] = useSelectedResources("ServerTemplate");
  return (
    <DataTable
      tableKey="server-templates"
      data={serverTemplates}
      selectOptions={{
        selectKey: ({ name }) => name,
        onSelect: setSelectedResources,
      }}
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
