import { TagsWithBadge } from "@components/tags";
import { useRead } from "@lib/hooks";
import { Icon } from "@radix-ui/react-select";
import { RequiredResourceComponents } from "@types";
import { Card, CardDescription, CardHeader, CardTitle } from "@ui/card";
import { DataTable } from "@ui/data-table";
import { GitBranch } from "lucide-react";
import { Link } from "react-router-dom";
import { RepoConfig } from "./config";

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
  Status: () => <></>,
  Page: {
    Config: RepoConfig,
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
