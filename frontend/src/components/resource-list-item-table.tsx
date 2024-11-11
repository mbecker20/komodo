import { useRead } from "@lib/hooks";
import { UsableResource } from "@types";
import { DataTable, SortableHeader } from "@ui/data-table";
import { Types } from "komodo_client";
import { ResourceComponents } from "./resources";
import { ResourceTagsV2 } from "./tags/tags-2";
import { ColumnDef } from "@tanstack/react-table";
import { ResourceLink } from "./resources/common";
import { HardDrive } from "lucide-react";
import { fmt_version } from "@lib/formatting";
import { BuilderInstanceType } from "./resources/builder";
import { StackComponents } from "./resources/stack";
import { useMemo } from "react";

export type ListItemInfoMap = {
  Deployment: Types.DeploymentListItem;
  Action: Types.ActionListItem;
  Builder: Types.BuilderListItem;
  Build: Types.BuildListItem;
  Alerter: Types.AlerterListItem;
  Procedure: Types.ProcedureListItem;
  Repo: Types.RepoListItem;
  ResourceSync: Types.ResourceSyncListItem;
  Server: Types.ServerListItem;
  ServerTemplate: Types.ServerTemplateListItem;
  Stack: Types.StackListItem;
};

export const ResourceListItemTable = <T extends keyof ListItemInfoMap>({
  type,
  data,
}: {
  type: UsableResource;
  data: ListItemInfoMap[T][];
}) => {
  const colums = useResourceTableColums(type);
  return (
    <DataTable
      tableKey={type + "-resources-table"}
      data={data}
      columns={colums}
    />
  );
};

const useResourceTableColums = <T extends UsableResource>(
  type: T
): ColumnDef<ListItemInfoMap[T]>[] => {
  const Components = ResourceComponents[type];
  return useMemo(
    () => [
      {
        accessorKey: "name",
        header: (h) => <SortableHeader column={h.column} title="Name" />,
        cell: ({ row: { original } }) => (
          <ResourceLink type={type} id={original.id} />
        ),
      },
      ...ExtraColumns[type],
      {
        accessorKey: "info.state",
        header: (h) => <SortableHeader column={h.column} title="State" />,
        cell: ({ row }) => <Components.State id={row.original.id} />,
        size: 120,
      },
      {
        accessorKey: "tags",
        header: (h) => <SortableHeader column={h.column} title="Tags" />,
        cell: ({ row }) => (
          <ResourceTagsV2
            target={{ type, id: row.original.id }}
            clickHandler="toggle"
          />
        ),
      },
    ],
    [type]
  );
};

const ExtraColumns: { [R in UsableResource]: ColumnDef<ListItemInfoMap[R]>[] } =
  {
    Action: [],
    Alerter: [],
    Build: [
      {
        accessorKey: "info.repo",
        size: 200,
        header: (h) => <SortableHeader column={h.column} title="Repo" />,
      },
      {
        header: "Version",
        size: 120,
        accessorFn: ({ info }) => fmt_version(info.version),
      },
    ],
    Builder: [
      {
        accessorKey: "info.builder_type",
        header: (h) => <SortableHeader column={h.column} title="Provider" />,
      },
      {
        accessorKey: "info.instance_type",
        header: (h) => (
          <SortableHeader column={h.column} title="Instance Type" />
        ),
        cell: ({ row }) => <BuilderInstanceType id={row.original.id} />,
      },
    ],
    Deployment: [
      {
        id: "image",
        size: 200,
        header: (h) => <SortableHeader column={h.column} title="Image" />,
        cell: ({ row: { original: o } }) => {
          const builds = useRead("ListBuilds", {}).data;
          const { build_id, image } = o.info;
          if (build_id) {
            const build = builds?.find((build) => build.id === build_id);
            if (build) {
              return <ResourceLink type="Build" id={build_id} />;
            } else {
              return "-- NOT-FOUND --";
            }
          } else {
            const [img] = image.split(":");
            return (
              <div className="flex gap-2 items-center">
                <HardDrive className="w-4 h-4" />
                {img}
              </div>
            );
          }
        },
      },
      {
        id: "server-id",
        size: 200,
        header: (h) => <SortableHeader column={h.column} title="Server" />,
        cell: ({ row }) => (
          <ResourceLink type="Server" id={row.original.info.server_id} />
        ),
        // sortingFn: (a, b) => {
        //   const sa = serverName(a.original.info.server_id);
        //   const sb = serverName(b.original.info.server_id);

        //   if (!sa && !sb) return 0;
        //   if (!sa) return -1;
        //   if (!sb) return 1;

        //   if (sa > sb) return 1;
        //   else if (sa < sb) return -1;
        //   else return 0;
        // },
      },
    ],
    Procedure: [
      {
        accessorKey: "info.stages",
        header: (h) => <SortableHeader column={h.column} title="Stages" />,
      },
    ],
    Repo: [
      {
        size: 200,
        accessorKey: "info.repo",
        header: (h) => <SortableHeader column={h.column} title="Repo" />,
      },
      {
        size: 200,
        accessorKey: "info.branch",
        header: (h) => <SortableHeader column={h.column} title="Branch" />,
      },
    ],
    ResourceSync: [
      {
        size: 200,
        accessorKey: "info.repo",
        header: (h) => <SortableHeader column={h.column} title="Repo" />,
      },
      {
        size: 200,
        accessorKey: "info.branch",
        header: (h) => <SortableHeader column={h.column} title="Branch" />,
      },
    ],
    Server: [
      // {
      //   accessorKey: "id",
      //   sortingFn: (a, b) => {
      //     const sa = resourcesCount(a.original.id);
      //     const sb = resourcesCount(b.original.id);

      //     if (!sa && !sb) return 0;
      //     if (!sa) return -1;
      //     if (!sb) return 1;

      //     if (sa > sb) return 1;
      //     else if (sa < sb) return -1;
      //     else return 0;
      //   },
      //   header: ({ column }) => (
      //     <SortableHeader column={column} title="Resources" />
      //   ),
      //   cell: ({ row }) => {
      //     return <>{resourcesCount(row.original.id)}</>;
      //   },
      // },
      {
        accessorKey: "info.region",
        header: ({ column }) => (
          <SortableHeader column={column} title="Region" />
        ),
      },
    ],
    ServerTemplate: [
      {
        accessorKey: "info.provider",
        header: (h) => <SortableHeader column={h.column} title="Provider" />,
      },
      {
        accessorKey: "info.instance_type",
        header: (h) => (
          <SortableHeader column={h.column} title="Instance Type" />
        ),
      },
    ],
    Stack: [
      {
        accessorKey: "info.server_id",
        // sortingFn: (a, b) => {
        //   const sa = serverName(a.original.info.server_id);
        //   const sb = serverName(b.original.info.server_id);

        //   if (!sa && !sb) return 0;
        //   if (!sa) return -1;
        //   if (!sb) return 1;

        //   if (sa > sb) return 1;
        //   else if (sa < sb) return -1;
        //   else return 0;
        // },
        header: (h) => <SortableHeader column={h.column} title="Server" />,
        cell: ({ row: { original } }) => (
          <ResourceLink type="Server" id={original.info.server_id} />
        ),
        size: 200,
      },
      {
        accessorKey: "info.state",
        header: (h) => <SortableHeader column={h.column} title="State" />,
        cell: ({ row }) => <StackComponents.State id={row.original.id} />,
        size: 120,
      },
    ],
  };
