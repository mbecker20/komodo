import { TagsWithBadge } from "@components/tags";
import { useRead, useTagsFilter } from "@lib/hooks";
import { RequiredResourceComponents } from "@types";
import { Card, CardDescription, CardHeader, CardTitle } from "@ui/card";
import { DataTable, SortableHeader } from "@ui/data-table";
import { FolderGit, GitBranch } from "lucide-react";
import { Link } from "react-router-dom";
import { RepoConfig } from "./config";
import { CloneRepo, PullRepo } from "./actions";
import { DeleteResource, NewResource, ResourceLink } from "../common";

const useRepo = (id?: string) =>
  useRead("ListRepos", {}).data?.find((d) => d.id === id);

export const RepoComponents: RequiredResourceComponents = {
  Dashboard: () => {
    const repo_count = useRead("ListRepos", {}).data?.length;
    return (
      <Link to="/repos/" className="w-full">
        <Card className="hover:bg-accent/50 transition-colors cursor-pointer">
          <CardHeader>
            <div className="flex justify-between">
              <div>
                <CardTitle>Repos</CardTitle>
                <CardDescription>{repo_count} Total</CardDescription>
              </div>
              <GitBranch className="w-4 h-4" />
            </div>
          </CardHeader>
        </Card>
      </Link>
    );
  },

  New: () => <NewResource type="Repo" />,

  Table: ({ search }) => {
    const tags = useTagsFilter();
    const repos = useRead("ListRepos", {}).data;
    const searchSplit = search?.split(" ") || [];
    return (
      <DataTable
        tableKey="repos"
        data={
          repos?.filter(
            (resource) =>
              tags.every((tag) => resource.tags.includes(tag)) &&
              (searchSplit.length > 0
                ? searchSplit.every((search) => resource.name.includes(search))
                : true)
          ) ?? []
        }
        columns={[
          {
            accessorKey: "name",
            header: ({ column }) => (
              <SortableHeader column={column} title="Name" />
            ),
            cell: ({ row }) => (
              <ResourceLink type="Repo" id={row.original.id} />
            ),
          },
          {
            header: "Repo",
            accessorKey: "info.repo",
          },
          {
            header: "Branch",
            accessorKey: "info.branch",
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

  Name: ({ id }: { id: string }) => <>{useRepo(id)?.name}</>,

  Icon: () => <GitBranch className="w-4 h-4" />,

  Status: {},

  Info: {
    Repo: ({ id }) => {
      const repo = useRepo(id)?.info.repo;
      return (
        <div className="flex items-center gap-2">
          <FolderGit className="w-4 h-4" />
          {repo}
        </div>
      );
    },
    Branch: ({ id }) => {
      const branch = useRepo(id)?.info.branch;
      return (
        <div className="flex items-center gap-2">
          <FolderGit className="w-4 h-4" />
          {branch}
        </div>
      );
    },
  },

  Actions: { PullRepo, CloneRepo },

  Page: {},

  Config: RepoConfig,

  DangerZone: ({ id }) => <DeleteResource type="Repo" id={id} />,
};
