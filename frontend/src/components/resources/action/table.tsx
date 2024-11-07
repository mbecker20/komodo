import { DataTable, SortableHeader } from "@ui/data-table";
import { TableTags } from "@components/tags";
import { ResourceLink } from "../common";
import { ActionComponents } from ".";
import { Types } from "komodo_client";
import { useSelectedResources } from "@lib/hooks";

export const ActionTable = ({
  actions,
}: {
  actions: Types.ActionListItem[];
}) => {
  const [_, setSelectedResources] = useSelectedResources("Action");

  return (
    <DataTable
      tableKey="actions"
      data={actions}
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
            <ResourceLink type="Action" id={row.original.id} />
          ),
        },
        {
          accessorKey: "info.state",
          header: ({ column }) => (
            <SortableHeader column={column} title="State" />
          ),
          cell: ({ row }) => <ActionComponents.State id={row.original.id} />,
        },
        {
          header: "Tags",
          cell: ({ row }) => <TableTags tag_ids={row.original.tags} />,
        },
      ]}
    />
  );
};
