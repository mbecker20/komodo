import { TagsWithBadge } from "@components/tags";
import { useFilterResources, useRead } from "@lib/hooks";
import { DataTable, SortableHeader } from "@ui/data-table";
import { fmt_version } from "@lib/formatting";
import { ResourceLink } from "../common";
import { BuildComponents } from ".";

export const BuildTable = ({ search }: { search?: string }) => {
  const builds = useRead("ListBuilds", {}).data;
  const filtered = useFilterResources(builds, search)
  return (
    <DataTable
      tableKey="builds"
      data={filtered}
      columns={[
        {
          accessorKey: "name",
          header: ({ column }) => (
            <SortableHeader column={column} title="Name" />
          ),
          cell: ({ row }) => <ResourceLink type="Build" id={row.original.id} />,
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
          header: "Version",
          accessorFn: ({ info }) => fmt_version(info.version),
          size: 120,
        },
        {
          accessorKey: "info.state",
          header: ({ column }) => (
            <SortableHeader column={column} title="State" />
          ),
          cell: ({ row }) => (
            <BuildComponents.Status.State id={row.original.id} />
          ),
          size: 120,
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
