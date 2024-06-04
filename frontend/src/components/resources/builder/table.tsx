import { useFilterResources, useRead } from "@lib/hooks";
import { DataTable, SortableHeader } from "@ui/data-table";
import { ResourceLink } from "../common";
import { TableTags } from "@components/tags";
import { BuilderInstanceType } from ".";

export const BuilderTable = ({ search }: { search?: string }) => {
  const builders = useRead("ListBuilders", {}).data;
  const filtered = useFilterResources(builders, search);
  return (
    <DataTable
      tableKey="builders"
      data={filtered}
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
