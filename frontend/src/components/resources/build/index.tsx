import { ConfigInner } from "@components/config";
import { ResourceSelector, AccountSelector } from "@components/config/util";
import { NewResource } from "@components/layouts";
import { ConfirmButton } from "@components/util";
import { useExecute, useRead, useWrite } from "@lib/hooks";
import { fmt_date_with_minutes, fmt_version } from "@lib/utils";
import { Types } from "@monitor/client";
import { RequiredResourceComponents } from "@types";
import { DataTable } from "@ui/data-table";
import { Input } from "@ui/input";
import { Hammer, History, Loader2 } from "lucide-react";
import { useState } from "react";
import { Link } from "react-router-dom";
import { ResourceComponents } from "..";
import { BuildChart } from "@components/dashboard/builds-chart";
import { useTagsFilter } from "@components/tags";

const useBuild = (id?: string) =>
  useRead("ListBuilds", {}).data?.find((d) => d.id === id);

const NewBuild = () => {
  const { mutateAsync } = useWrite("CreateBuild");
  const [name, setName] = useState("");
  return (
    <NewResource
      type="Build"
      onSuccess={() => mutateAsync({ name, config: {} })}
      enabled={!!name}
    >
      <div className="grid md:grid-cols-2">
        Build Name
        <Input
          placeholder="build-name"
          value={name}
          onChange={(e) => setName(e.target.value)}
        />
      </div>
    </NewResource>
  );
};

export const BuildConfig = ({ id }: { id: string }) => {
  const config = useRead("GetBuild", { id }).data?.config;
  const [update, set] = useState<Partial<Types.BuildConfig>>({});
  const { mutate } = useWrite("UpdateBuild");

  if (!config) return null;

  return (
    <ConfigInner
      config={config}
      update={update}
      set={set}
      onSave={() => mutate({ id, config: update })}
      components={{
        general: {
          general: {
            builder_id: (id, set) => (
              <div className="flex justify-between items-center border-b pb-4 min-h-[60px]">
                <div>Builder</div>
                <ResourceSelector
                  type="Builder"
                  selected={id}
                  onSelect={(builder_id) => set({ builder_id })}
                />
              </div>
            ),
            repo: true,
            branch: true,
            github_account: (account, set) => (
              <AccountSelector
                id={update.builder_id ?? config.builder_id ?? undefined}
                type="Builder"
                account_type="github"
                selected={account}
                onSelect={(github_account) => set({ github_account })}
              />
            ),
          },
        },
        docker: {
          docker: {
            build_path: true,
            dockerfile_path: true,
            docker_account: (account, set) => (
              <AccountSelector
                id={update.builder_id ?? config.builder_id ?? undefined}
                type="Builder"
                account_type="docker"
                selected={account}
                onSelect={(docker_account) => set({ docker_account })}
              />
            ),
            use_buildx: true,
            // docker_organization,
          },
        },
      }}
    />
  );
};

const Name = ({ id }: { id: string }) => <>{useBuild(id)?.name}</>;

const Icon = ({ id }: { id: string }) => {
  const building = useRead("GetBuildActionState", { id }).data?.building;
  const className = building
    ? "w-4 h-4 animate-spin fill-green-500"
    : "w-4 h-4";
  return <Hammer className={className} />;
};

const BuildTable = () => {
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
          header: "Version",
          accessorFn: ({ info }) => {
            return fmt_version(info.version);
          },
        },
        // {
        //   header: "Deployments",
        //   cell: ({ row }) => {
        //     const deps = useRead("ListDeployments", {
        //       query: { specific: { build_ids: [row.original.id] } },
        //     })?.data?.map((d) => (
        //       <Link to={`/deployments/${d.id}`}>{d.name}</Link>
        //     ));
        //     return <div className="flex items-center gap-2">{deps}</div>;
        //   },
        // },
        { header: "Tags", accessorFn: ({ tags }) => tags.join(", ") },
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
          header: "Created",
          accessorFn: ({ created_at }) =>
            fmt_date_with_minutes(new Date(created_at)),
        },
      ]}
    />
  );
};

export const BuildComponents: RequiredResourceComponents = {
  Name,
  Description: ({ id }) => <>{fmt_version(useBuild(id)?.info.version)}</>,
  Info: ({ id }) => {
    const ts = useBuild(id)?.info.last_built_at;
    return (
      <div className="flex items-center gap-2">
        <History className="w-4 h-4" />
        {ts ? new Date(ts).toLocaleString() : "Never Built"}
      </div>
    );
  },
  Page: {
    Config: ({ id }) => <BuildConfig id={id} />,
  },
  Icon: ({ id }) => {
    if (id) return <Icon id={id} />;
    else return <Hammer className="w-4 h-4" />;
  },
  Actions: ({ id }) => {
    const building = useRead("GetBuildActionState", { id }).data?.building;
    const { mutate, isPending } = useExecute("RunBuild");
    return (
      <ConfirmButton
        title={building ? "Building" : "Build"}
        icon={
          building ? (
            <Loader2 className="w-4 h-4 animate-spin" />
          ) : (
            <Hammer className="h-4 w-4" />
          )
        }
        onClick={() => mutate({ build_id: id })}
        disabled={building || isPending}
      />
    );
  },
  Table: BuildTable,
  New: NewBuild,
  Dashboard: BuildChart,
};
