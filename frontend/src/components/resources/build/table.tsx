import { TagsWithBadge } from "@components/tags";
import { useRead, useTagsFilter } from "@lib/hooks";
import { DataTable, SortableHeader } from "@ui/data-table";
import { fmt_version } from "@lib/formatting";
import { ResourceLink } from "../common";
import { BuildComponents } from ".";

export const BuildTable = ({ search }: { search?: string }) => {
  const builds = useRead("ListBuilds", {}).data;
  const tags = useTagsFilter();
  const searchSplit = search?.split(" ") || [];
  return (
    <DataTable
      tableKey="builds"
      data={
        builds?.filter(
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
          cell: ({ row }) => <ResourceLink type="Build" id={row.original.id} />,
        },
        {
          header: "Version",
          accessorFn: ({ info }) => fmt_version(info.version),
        },
        {
          accessorKey: "info.repo",
          header: ({ column }) => (
            <SortableHeader column={column} title="Repo" />
          ),
        },
        {
          accessorKey: "info.status",
          header: ({ column }) => (
            <SortableHeader column={column} title="Status" />
          ),
          cell: ({ row }) => (
            <BuildComponents.Status.Status id={row.original.id} />
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
