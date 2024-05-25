import { useFilterResources } from "@lib/hooks";
import { DataTable, SortableHeader } from "@ui/data-table";
import { ResourceLink } from "../common";
import { TagsWithBadge } from "@components/tags";
import { RepoComponents } from ".";
import { Types } from "@monitor/client";

export const RepoTable = ({
  repos,
  search,
}: {
  search?: string;
  repos: Types.RepoListItem[] | undefined;
}) => {
  // const servers = useRead("ListServers", {}).data;
  // const serverName = useCallback(
  //   (id: string) => servers?.find((server) => server.id === id)?.name,
  //   [servers]
  // );
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
            <RepoComponents.Status.State id={row.original.id} />
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
