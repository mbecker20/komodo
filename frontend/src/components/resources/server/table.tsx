import { TagsWithBadge, useTagsFilter } from "@components/tags";
import { useRead } from "@lib/hooks";
import { DataTable } from "@ui/data-table";
import { ServerComponents } from ".";
import { ResourceComponents } from "..";

export const ServerTable = () => {
  const servers = useRead("ListServers", {}).data;
  const tags = useTagsFilter();
  return (
    <DataTable
      // onRowClick={({ id }) => nav(`/servers/${id}`)}
      data={
        servers?.filter((server) =>
          tags.every((tag) => server.tags.includes(tag))
        ) ?? []
      }
      columns={[
        {
          header: "Name",
          accessorKey: "id",
          cell: ({ row }) => {
            return <ResourceComponents.Server.Link id={row.original.id} />;
          },
        },
        {
          header: "Deployments",
          cell: ({ row }) => <DeploymentCountOnServer id={row.original.id} />,
        },
        { header: "Region", accessorKey: "info.region" },
        {
          header: "State",
          cell: ({
            row: {
              original: { id },
            },
          }) => <ServerComponents.Status id={id} />,
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

const DeploymentCountOnServer = ({ id }: { id: string }) => {
  const { data } = useRead("ListDeployments", {
    query: { specific: { server_ids: [id] } },
  });

  return <>{data?.length ?? 0}</>;
};
