import { TagsWithBadge, useTagsFilter } from "@components/tags";
import { useRead, useWrite } from "@lib/hooks";
import { RequiredResourceComponents } from "@types";
import { Card, CardDescription, CardHeader, CardTitle } from "@ui/card";
import { DataTable } from "@ui/data-table";
import { AlertTriangle, FolderGit, GitBranch, History } from "lucide-react";
import { Link } from "react-router-dom";
import { RepoConfig } from "./config";
import { useState } from "react";
import { NewResource, Section } from "@components/layouts";
import { Input } from "@ui/input";
import { CloneRepo, PullRepo } from "./actions";
import { CopyResource, DeleteResource, ResourceLink } from "../common";

const useRepo = (id?: string) =>
  useRead("ListRepos", {}).data?.find((d) => d.id === id);

export const RepoComponents: RequiredResourceComponents = {
  Name: ({ id }: { id: string }) => <>{useRepo(id)?.name}</>,
  Icon: () => <GitBranch className="w-4 h-4" />,
  Link: ({ id }) => <ResourceLink type="Repo" id={id} />,
  Info: [
    ({ id }) => {
      const repo = useRepo(id)?.info.repo;
      return (
        <div className="flex items-center gap-2">
          <FolderGit className="w-4 h-4" />
          {repo}
        </div>
      );
    },
    ({ id }) => {
      const ts = useRepo(id)?.info.last_pulled_at;
      return (
        <div className="flex items-center gap-2">
          <History className="w-4 h-4" />
          {ts ? new Date(ts).toLocaleString() : "Never Pulled"}
        </div>
      );
    },
  ],
  Status: () => <>Repo</>,
  Actions: [PullRepo, CloneRepo],
  Page: {
    Config: RepoConfig,
    Danger: ({ id }) => (
      <Section
        title="Danger Zone"
        icon={<AlertTriangle className="w-4 h-4" />}
        actions={<CopyResource type="Repo" id={id} />}
      >
        <DeleteResource type="Repo" id={id} />
      </Section>
    ),
  },
  Table: ({ search }) => {
    const tags = useTagsFilter();
    const repos = useRead("ListRepos", {}).data;
    const searchSplit = search?.split(" ") || [];
    return (
      <DataTable
        data={
          repos?.filter((resource) =>
            tags.every((tag) => resource.tags.includes(tag)) &&
            searchSplit.length > 0
              ? searchSplit.every((search) => resource.name.includes(search))
              : true
          ) ?? []
        }
        columns={[
          {
            header: "Name",
            cell: ({ row }) => <RepoComponents.Link id={row.original.id} />,
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
  New: () => {
    const { mutateAsync } = useWrite("CreateRepo");
    const [name, setName] = useState("");
    return (
      <NewResource
        entityType="Repo"
        onSuccess={() => mutateAsync({ name, config: {} })}
        enabled={!!name}
      >
        <div className="grid md:grid-cols-2">
          Repo Name
          <Input
            placeholder="repo-name"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
        </div>
      </NewResource>
    );
  },
};
