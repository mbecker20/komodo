import { TagsWithBadge } from "@components/tags";
import { useRead, useTagsFilter } from "@lib/hooks";
import { DataTable } from "@ui/data-table";
import { ServerComponents } from ".";
import { ResourceLink } from "../common";

export const ServerTable = ({ search }: { search?: string }) => {
  const servers = useRead("ListServers", {}).data;
  const tags = useTagsFilter();
  const searchSplit = search?.split(" ") || [];
  return (
    <DataTable
      data={
        servers?.filter(
          (resource) =>
            tags.every((tag) => resource.tags.includes(tag)) &&
            (searchSplit.length > 0
              ? searchSplit.every((search) => resource.name.includes(search))
              : true)
        ) ?? []
      }
      columns={[
        {
          header: "Name",
          accessorKey: "id",
          cell: ({ row }) => {
            return <ResourceLink type="Server" id={row.original.id} />;
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
          }) => {
            return <ServerComponents.Status.Status id={id} />;
          },
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
