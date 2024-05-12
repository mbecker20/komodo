import { useFilterResources, useRead } from "@lib/hooks";
import { DataTable, SortableHeader } from "@ui/data-table";
import { ResourceLink } from "../common";
import { TagsWithBadge } from "@components/tags";
import { RepoComponents } from ".";

export const RepoTable = ({ search }: { search?: string }) => {
  const repos = useRead("ListRepos", {}).data;
  const filtered = useFilterResources(repos, search);
  return (
    <DataTable
      tableKey="repos"
      data={filtered}
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
          accessorKey: "info.state",
          header: ({ column }) => (
            <SortableHeader column={column} title="State" />
          ),
          cell: ({ row }) => (
            <RepoComponents.Status.State id={row.original.id} />
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
