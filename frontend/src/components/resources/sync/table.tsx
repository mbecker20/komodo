import { DataTable, SortableHeader } from "@ui/data-table";
import { ResourceLink } from "../common";
import { TableTags } from "@components/tags";
import { Types } from "@monitor/client";

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
        },
        // {
        //   accessorKey: "info.builder_type",
        //   header: ({ column }) => (
        //     <SortableHeader column={column} title="Provider" />
        //   ),
        // },
        // {
        //   accessorKey: "info.instance_type",
        //   header: ({ column }) => (
        //     <SortableHeader column={column} title="Instance Type" />
        //   ),
        //   cell: ({ row }) => <BuilderInstanceType id={row.original.id} />,
        // },
        {
          header: "Tags",
          cell: ({ row }) => <TableTags tag_ids={row.original.tags} />,
        },
      ]}
    />
  );
};
