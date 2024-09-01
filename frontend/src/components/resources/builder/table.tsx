import { DataTable, SortableHeader } from "@ui/data-table";
import { ResourceLink } from "../common";
import { TableTags } from "@components/tags";
import { BuilderInstanceType } from ".";
import { Types } from "@komodo/client";

export const BuilderTable = ({
  builders,
}: {
  builders: Types.BuilderListItem[];
}) => {
  return (
    <DataTable
      tableKey="builders"
      data={builders}
      columns={[
        {
          accessorKey: "name",
          header: ({ column }) => (
            <SortableHeader column={column} title="Name" />
          ),
          cell: ({ row }) => (
            <ResourceLink type="Builder" id={row.original.id} />
          ),
        },
        {
          accessorKey: "info.builder_type",
          header: ({ column }) => (
            <SortableHeader column={column} title="Provider" />
          ),
        },
        {
          accessorKey: "info.instance_type",
          header: ({ column }) => (
            <SortableHeader column={column} title="Instance Type" />
          ),
          cell: ({ row }) => <BuilderInstanceType id={row.original.id} />,
        },
        {
          header: "Tags",
          cell: ({ row }) => <TableTags tag_ids={row.original.tags} />,
        },
      ]}
    />
  );
};
