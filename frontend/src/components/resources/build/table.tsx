import { TagsWithBadge, useTagsFilter } from "@components/tags";
import { useRead } from "@lib/hooks";
import { DataTable } from "@ui/data-table";
import { Link } from "react-router-dom";
import { ResourceComponents } from "..";
import { fmt_date_with_minutes, fmt_version } from "@lib/formatting";

export const BuildTable = () => {
  const builds = useRead("ListBuilds", {}).data;
  const tags = useTagsFilter();
  return (
    <DataTable
      data={
        builds?.filter((build) =>
          tags.every((tag) => build.tags.includes(tag))
        ) ?? []
      }
      columns={[
        {
          accessorKey: "id",
          header: "Name",
          cell: ({ row }) => {
            const id = row.original.id;
            return (
              <Link to={`/builds/${id}`} className="flex items-center gap-2">
                <ResourceComponents.Build.Icon id={id} />
                <ResourceComponents.Build.Name id={id} />
              </Link>
            );
          },
        },
        {
          header: "Repo",
          accessorKey: "info.repo",
        },
        {
          header: "Version",
          accessorFn: ({ info }) => fmt_version(info.version),
        },
        {
          header: "Last Built",
          accessorFn: ({ info: { last_built_at } }) => {
            if (last_built_at > 0) {
              return fmt_date_with_minutes(new Date(last_built_at));
            } else {
              return "never";
            }
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
