import { DataTable, SortableHeader } from "@ui/data-table";
import { ResourceLink } from "../common";
import { TableTags } from "@components/tags";
import { RepoComponents } from ".";
import { Types } from "komodo_client";
import { useSelectedResources } from "@lib/hooks";

export const RepoTable = ({ repos }: { repos: Types.RepoListItem[] }) => {
  const [_, setSelectedResources] = useSelectedResources("Repo");

  return (
    <DataTable
      tableKey="repos"
      data={repos}
      selectOptions={{
        selectKey: ({ id }) => id,
        onSelect: setSelectedResources,
      }}
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
          cell: ({ row }) => <RepoComponents.State id={row.original.id} />,
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
