import { useRead, useTagsFilter } from "@lib/hooks";
import { DataTable, SortableHeader } from "@ui/data-table";
import { ResourceLink } from "../common";
import { TagsWithBadge } from "@components/tags";
import { RepoComponents } from ".";

export const RepoTable = ({ search }: { search?: string }) => {
  const tags = useTagsFilter();
  const repos = useRead("ListRepos", {}).data;
  const searchSplit = search?.split(" ") || [];
  return (
    <DataTable
      tableKey="repos"
      data={
        repos?.filter(
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
          cell: ({ row }) => <ResourceLink type="Repo" id={row.original.id} />,
        },
        {
          accessorKey: "info.repo",
          header: ({ column }) => (
            <SortableHeader column={column} title="Repo" />
          ),
        },
        {
          accessorKey: "info.branch",
          header: ({ column }) => (
            <SortableHeader column={column} title="Branch" />
          ),
        },
        {
          accessorKey: "info.status",
          header: ({ column }) => (
            <SortableHeader column={column} title="Status" />
          ),
          cell: ({ row }) => (
            <RepoComponents.Status.Status id={row.original.id} />
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
