import { NewResource } from "@components/layouts";
import { ConfirmButton } from "@components/util";
import { useExecute, useRead, useWrite } from "@lib/hooks";
import { fmt_date_with_minutes, fmt_version } from "@lib/utils";
import { RequiredResourceComponents } from "@types";
import { DataTable } from "@ui/data-table";
import { Input } from "@ui/input";
import { Ban, Hammer, History, Loader2 } from "lucide-react";
import { useState } from "react";
import { Link } from "react-router-dom";
import { ResourceComponents } from "..";
import { BuildChart } from "@components/dashboard/builds-chart";
import { TagsWithBadge, useTagsFilter } from "@components/tags";
import { useToast } from "@ui/use-toast";
import { BuildConfig } from "./config";

const useBuild = (id?: string) =>
  useRead("ListBuilds", {}).data?.find((d) => d.id === id);

const NewBuild = () => {
  const { mutateAsync } = useWrite("CreateBuild");
  const [name, setName] = useState("");
  return (
    <NewResource
      entityType="Build"
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

const Name = ({ id }: { id: string }) => <>{useBuild(id)?.name}</>;

const Icon = ({ id }: { id: string }) => {
  const building = useRead("GetBuildActionState", { build: id }).data?.building;
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
          header: "Repo",
          accessorKey: "info.repo"
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
  Status: () => <>Build</>,
  Page: {
    Config: ({ id }) => <BuildConfig id={id} />,
  },
  Icon: ({ id }) => {
    if (id) return <Icon id={id} />;
    else return <Hammer className="w-4 h-4" />;
  },
  Actions: ({ id }) => {
    const { toast } = useToast();
    const building = useRead("GetBuildActionState", { build: id }).data
      ?.building;
    const { mutate: run_mutate, isPending: runPending } = useExecute(
      "RunBuild",
      {
        onMutate: () => {
          toast({ title: "Run Build Sent" });
        },
      }
    );
    const { mutate: cancel_mutate, isPending: cancelPending } = useExecute(
      "CancelBuild",
      {
        onMutate: () => {
          toast({ title: "Cancel Build Sent" });
        },
        onSuccess: () => {
          toast({ title: "Build Cancelled" });
        },
      }
    );
    if (building) {
      return (
        <ConfirmButton
          title="Cancel Build"
          variant="destructive"
          icon={<Ban className="h-4 w-4" />}
          onClick={() => cancel_mutate({ build: id })}
          disabled={cancelPending}
        />
      );
    } else {
      return (
        <ConfirmButton
          title="Build"
          icon={
            runPending ? (
              <Loader2 className="w-4 h-4 animate-spin" />
            ) : (
              <Hammer className="h-4 w-4" />
            )
          }
          onClick={() => run_mutate({ build: id })}
          disabled={runPending}
        />
      );
    }
  },
  Table: BuildTable,
  New: NewBuild,
  Dashboard: BuildChart,
};
