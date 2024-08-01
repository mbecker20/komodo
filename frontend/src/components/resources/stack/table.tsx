import { useRead } from "@lib/hooks";
import { DataTable, SortableHeader } from "@ui/data-table";
import { ResourceLink } from "../common";
import { TableTags } from "@components/tags";
import { StackComponents } from ".";
import { Types } from "@monitor/client";
import { useCallback } from "react";

export const StackTable = ({ stacks }: { stacks: Types.StackListItem[] }) => {
  const servers = useRead("ListServers", {}).data;
  const serverName = useCallback(
    (id: string) => servers?.find((server) => server.id === id)?.name,
    [servers]
  );
  return (
    <DataTable
      tableKey="Stacks"
      data={stacks}
      columns={[
        {
          accessorKey: "name",
          header: ({ column }) => (
            <SortableHeader column={column} title="Name" />
          ),
          cell: ({ row }) => <ResourceLink type="Stack" id={row.original.id} />,
          size: 200,
        },
        {
          accessorKey: "info.server_id",
          sortingFn: (a, b) => {
            const sa = serverName(a.original.info.server_id);
            const sb = serverName(b.original.info.server_id);

            if (!sa && !sb) return 0;
            if (!sa) return -1;
            if (!sb) return 1;

            if (sa > sb) return 1;
            else if (sa < sb) return -1;
            else return 0;
          },
          header: ({ column }) => (
            <SortableHeader column={column} title="Server" />
          ),
          cell: ({ row }) => (
            <ResourceLink type="Server" id={row.original.info.server_id} />
          ),
          size: 200,
        },
        {
          accessorKey: "info.state",
          header: ({ column }) => (
            <SortableHeader column={column} title="State" />
          ),
          cell: ({ row }) => (
            <StackComponents.Status.State id={row.original.id} />
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
