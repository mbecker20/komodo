import { TagsWithBadge, useTagsFilter } from "@components/tags";
import { Types } from "@monitor/client";
import { DataTable } from "@ui/data-table";
import { useRead } from "@lib/hooks";
import { ResourceComponents } from "..";
import {
  deployment_state_intention,
  text_color_class_by_intention,
} from "@lib/color";
import { snake_case_to_upper_space_case } from "@lib/formatting";

export const DeploymentTable = ({
  deployments,
  search,
}: {
  deployments: Types.DeploymentListItem[] | undefined;
  search?: string;
}) => {
  const tags = useTagsFilter();
  const searchSplit = search?.split(" ") || [];
  return (
    <DataTable
      data={
        deployments?.filter((resource) =>
          tags.every((tag) => resource.tags.includes(tag)) &&
          searchSplit.length > 0
            ? searchSplit.every((search) => resource.name.includes(search))
            : true
        ) ?? []
      }
      columns={[
        {
          header: "Name",
          cell: ({ row }) => (
            <ResourceComponents.Deployment.Link id={row.original.id} />
          ),
        },
        {
          header: "Image",
          cell: ({
            row: {
              original: {
                info: { build_id, image },
              },
            },
          }) => {
            const builds = useRead("ListBuilds", {}).data;
            if (build_id) {
              const build = builds?.find((build) => build.id === build_id);
              if (build) {
                return <ResourceComponents.Build.Link id={build_id} />;
              } else {
                return undefined;
              }
            } else {
              const [img] = image.split(":");
              return img;
            }
          },
        },
        {
          header: "Server",
          cell: ({ row }) => (
            <ResourceComponents.Server.Link id={row.original.info.server_id} />
          ),
        },
        {
          header: "State",
          cell: ({ row }) => {
            const state = row.original.info.state;
            const color = text_color_class_by_intention(
              deployment_state_intention(state)
            );
            return (
              <div className={color}>
                {snake_case_to_upper_space_case(state)}
              </div>
            );
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
