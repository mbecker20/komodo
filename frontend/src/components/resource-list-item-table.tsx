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
import { useCallback, useMemo } from "react";

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

  const RESOURCE_SPECIFIC_COLUMNS = useResourceSpecificColumns(type);

  return useMemo(
    () => [
      {
        size: 200,
        accessorKey: "name",
        header: (h) => <SortableHeader column={h.column} title="Name" />,
        cell: ({ row: { original } }) => (
          <ResourceLink type={type} id={original.id} />
        ),
      },
      ...RESOURCE_SPECIFIC_COLUMNS,
      {
        size: 100,
        accessorKey: "info.state",
        header: (h) => <SortableHeader column={h.column} title="State" />,
        cell: ({ row }) => <Components.State id={row.original.id} />,
      },
      {
        size: 200,
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

const useResourceSpecificColumns = <T extends UsableResource>(
  type: T
): ColumnDef<ListItemInfoMap[T]>[] => {
  const sortServerIdsByName = useSortServerIdsByName(
    type === "Deployment" || type === "Stack"
  );

  const countResources = useCountServerResources(type === "Server");
  const sortServerIdsByResourceCount = useSortServerIdByResourceCount(
    type === "Server"
  );

  return useMemo(() => {
    const ExtraColumns: {
      [R in UsableResource]: ColumnDef<ListItemInfoMap[R]>[];
    } = {
      Action: [
        {
          size: 200,
          id: "last-run-at",
          header: (h) => <SortableHeader column={h.column} title="Repo" />,
          cell: ({ row }) =>
            new Date(row.original.info.last_run_at).toLocaleString(),
        },
      ],
      Alerter: [],
      Build: [
        {
          accessorKey: "info.repo",
          size: 200,
          header: (h) => <SortableHeader column={h.column} title="Repo" />,
        },
        {
          header: "Version",
          size: 200,
          accessorFn: ({ info }) => fmt_version(info.version),
        },
      ],
      Builder: [
        {
          size: 200,
          accessorKey: "info.builder_type",
          header: (h) => <SortableHeader column={h.column} title="Provider" />,
        },
        {
          size: 200,
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
          cell: ({ row }) => <DeploymentImage deployment={row.original} />,
        },
        {
          id: "server-id",
          size: 200,
          header: (h) => <SortableHeader column={h.column} title="Server" />,
          cell: ({ row }) => (
            <ResourceLink type="Server" id={row.original.info.server_id} />
          ),
          sortingFn: (a, b) =>
            sortServerIdsByName(
              a.original.info.server_id,
              b.original.info.server_id
            ),
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
        {
          id: "count",
          header: (h) => <SortableHeader column={h.column} title="Resources" />,
          cell: ({ row }) => <>{countResources(row.original.id)}</>,
          sortingFn: (a, b) =>
            sortServerIdsByResourceCount(a.original.id, b.original.id),
        },
        {
          accessorKey: "info.region",
          header: ({ column }) => (
            <SortableHeader column={column} title="Region" />
          ),
        },
      ],
      ServerTemplate: [
        {
          size: 200,
          accessorKey: "info.provider",
          header: (h) => <SortableHeader column={h.column} title="Provider" />,
        },
        {
          size: 200,
          accessorKey: "info.instance_type",
          header: (h) => (
            <SortableHeader column={h.column} title="Instance Type" />
          ),
        },
      ],
      Stack: [
        {
          accessorKey: "info.server_id",
          header: (h) => <SortableHeader column={h.column} title="Server" />,
          cell: ({ row: { original } }) => (
            <ResourceLink type="Server" id={original.info.server_id} />
          ),
          size: 200,
          sortingFn: (a, b) =>
            sortServerIdsByName(
              a.original.info.server_id,
              b.original.info.server_id
            ),
        },
      ],
    };

    return ExtraColumns[type];
  }, [type, sortServerIdsByName]);
};

const useSortServerIdsByName = (enabled: boolean) => {
  const servers = useRead("ListServers", {}, { enabled }).data;

  return useCallback(
    (...args: [id1: string, id2: string]) => {
      const [a, b] = args.map((id) => servers?.find((s) => s.id === id)?.name);

      if (!a && !b) return 0;

      if (!a) return -1;
      if (!b) return 1;

      return a === b ? 0 : a > b ? 1 : -1;
    },
    [servers]
  );
};

const useCountServerResources = (enabled: boolean) => {
  const deployments = useRead(`ListDeployments`, {}, { enabled }).data;
  const repos = useRead(`ListRepos`, {}, { enabled }).data;
  const stacks = useRead(`ListStacks`, {}, { enabled }).data;

  return useCallback(
    (id: string) => {
      return [deployments, repos, stacks]
        .map(
          (resources) =>
            resources?.filter((r) => r.info.server_id === id).length ?? 0
        )
        .reduce((sum, num) => (sum += num), 0);
    },
    [deployments, repos, stacks]
  );
};

const useSortServerIdByResourceCount = (enabled: boolean) => {
  const count = useCountServerResources(enabled);

  return useCallback(
    (...args: [id1: string, id2: string]) => {
      const [a, b] = args.map((id) => count(id));

      if (!a && !b) return 0;

      if (!a) return -1;
      if (!b) return 1;

      return a === b ? 0 : a > b ? 1 : -1;
    },
    [count]
  );
};

const DeploymentImage = ({
  deployment,
}: {
  deployment: Types.DeploymentListItem;
}) => {
  const { build_id, image } = deployment.info;
  const builds = useRead("ListBuilds", {}, { enabled: !!build_id }).data;

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
      <div className="flex gap-2 items-center whitespace-nowrap">
        <HardDrive className="w-4 h-4" />
        {img}
      </div>
    );
  }
};
