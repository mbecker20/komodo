import { TagsWithBadge, useTagsFilter } from "@components/tags";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { DataTable } from "@ui/data-table";
import { Link } from "react-router-dom";
import { useRead } from "@lib/hooks";
import { ResourceComponents } from "..";
import {
  deployment_state_intention,
  text_color_class_by_intention,
} from "@lib/color";
import { snake_case_to_upper_space_case } from "@lib/formatting";

export const DeploymentTable = ({
  deployments,
}: {
  deployments: Types.DeploymentListItem[] | undefined;
}) => {
  const tags = useTagsFilter();
  return (
    <DataTable
      data={
        deployments?.filter((deployment) =>
          tags.every((tag) => deployment.tags.includes(tag))
        ) ?? []
      }
      columns={[
        {
          header: "Name",
          cell: ({ row }) => {
            const id = row.original.id;
            return (
              <Link to={`/deployments/${id}`}>
                <Button variant="link" className="flex gap-2 items-center p-0">
                  <ResourceComponents.Deployment.Icon id={id} />
                  <ResourceComponents.Deployment.Name id={id} />
                </Button>
              </Link>
            );
          },
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
                return (
                  <Link to={`/builds/${build_id}`}>
                    <Button
                      variant="link"
                      className="flex gap-2 items-center p-0"
                    >
                      <ResourceComponents.Build.Icon id={build_id} />
                      <ResourceComponents.Build.Name id={build_id} />
                    </Button>
                  </Link>
                );
              } else {
                return undefined;
              }
            } else {
              const [img, _] = image.split(":");
              return img;
            }
          },
        },
        {
          header: "Server",
          cell: ({ row }) => {
            const id = row.original.info.server_id;
            return (
              <Link to={`/servers/${id}`}>
                <Button variant="link" className="flex items-center gap-2 p-0">
                  <ResourceComponents.Server.Icon id={id} />
                  <ResourceComponents.Server.Name id={id} />
                </Button>
              </Link>
            );
          },
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
