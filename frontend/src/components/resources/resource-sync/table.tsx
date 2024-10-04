import { DataTable, SortableHeader } from "@ui/data-table";
import { ResourceLink } from "../common";
import { TableTags } from "@components/tags";
import { Types } from "@komodo/client";
import { ResourceSyncComponents } from ".";

export const ResourceSyncTable = ({
  syncs,
}: {
  syncs: Types.ResourceSyncListItem[];
}) => {
  return (
    <DataTable
      tableKey="syncs"
      data={syncs}
      columns={[
        {
          accessorKey: "name",
          header: ({ column }) => (
            <SortableHeader column={column} title="Name" />
          ),
          cell: ({ row }) => (
            <ResourceLink type="ResourceSync" id={row.original.id} />
          ),
          size: 200,
        },
        {
          accessorKey: "info.repo",
          header: ({ column }) => (
            <SortableHeader column={column} title="Repo" />
          ),
          size: 200,
        },
        {
          accessorKey: "info.branch",
          header: ({ column }) => (
            <SortableHeader column={column} title="Branch" />
          ),
          size: 200,
        },
        {
          accessorKey: "info.state",
          header: ({ column }) => (
            <SortableHeader column={column} title="State" />
          ),
          cell: ({ row }) => (
            <ResourceSyncComponents.State id={row.original.id} />
          ),
          size: 120,
        },
        {
          header: "Tags",
          cell: ({ row }) => <TableTags tag_ids={row.original.tags} />,
        },
      ]}
    />
  );
};
