import { Config } from "@components/config";
import { AccountSelector, ResourceSelector } from "@components/config/util";
import { TagsWithBadge } from "@components/tags";
import { useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { Icon } from "@radix-ui/react-select";
import { RequiredResourceComponents } from "@types";
import { Card, CardDescription, CardHeader, CardTitle } from "@ui/card";
import { DataTable } from "@ui/data-table";
import { GitBranch } from "lucide-react";
import { useState } from "react";
import { Link } from "react-router-dom";

const useRepo = (id?: string) =>
  useRead("ListRepos", {}).data?.find((d) => d.id === id);

const Name = ({ id }: { id: string }) => <>{useRepo(id)?.name}</>;

export const RepoDashboard = () => {
  const repo_count = useRead("ListRepos", {}).data?.length;
  return (
    <Link to="/repos/" className="w-full">
      <Card>
        <CardHeader className="justify-between">
          <div>
            <CardTitle>Repos</CardTitle>
            <CardDescription>{repo_count} Total</CardDescription>
          </div>
          <GitBranch className="w-4 h-4" />
        </CardHeader>
      </Card>
    </Link>
  );
};

export const RepoComponents: RequiredResourceComponents = {
  Name,
  Description: ({ id }) => <>{id}</>,
  Info: ({ id }) => <>{id}</>,
  Icon: () => <GitBranch className="w-4 h-4" />,
  Page: {
    Config: ({ id }) => {
      const config = useRead("GetRepo", { repo: id }).data?.config;
      const [update, set] = useState<Partial<Types.RepoConfig>>({});
      const mutate = useWrite("UpdateRepo");
      if (!config) return null;
      return (
        <Config
          config={config}
          update={update}
          set={set}
          onSave={() => mutate}
          components={{
            general: {
              general: {
                server_id: (selected, set) => (
                  <ResourceSelector
                    type="Server"
                    selected={selected}
                    onSelect={(server_id) => set({ server_id })}
                  />
                ),
                github_account: (value, set) => (
                  <AccountSelector
                    type="Server"
                    account_type="github"
                    id={update.server_id ?? config.server_id}
                    selected={value}
                    onSelect={(github_account) => set({ github_account })}
                  />
                ),
                repo: true,
                branch: true,
                on_pull: true,
                on_clone: true,
              },
            },
          }}
        />
      );
    },
  },
  Table: () => {
    const alerters = useRead("ListAlerters", {}).data;
    return (
      <DataTable
        data={alerters ?? []}
        columns={[
          {
            accessorKey: "id",
            header: "Name",
            cell: ({ row }) => {
              const id = row.original.id;
              return (
                <Link to={`/repos/${id}`} className="flex items-center gap-2">
                  <Icon id={id} />
                  <Name id={id} />
                </Link>
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
  },
  Actions: () => null,
  New: () => null,
  Dashboard: RepoDashboard,
};
