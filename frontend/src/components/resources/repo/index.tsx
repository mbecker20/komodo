import { TagsWithBadge } from "@components/tags";
import { useRead, useWrite } from "@lib/hooks";
import { RequiredResourceComponents } from "@types";
import { Card, CardDescription, CardHeader, CardTitle } from "@ui/card";
import { DataTable } from "@ui/data-table";
import { AlertTriangle, GitBranch } from "lucide-react";
import { Link } from "react-router-dom";
import { DeleteRepo, RepoConfig } from "./config";
import { CopyResource, ResourceLink } from "@components/util";
import { useState } from "react";
import { NewResource, Section } from "@components/layouts";
import { Input } from "@ui/input";
import { CloneRepo, PullRepo } from "./actions";

const useRepo = (id?: string) =>
  useRead("ListRepos", {}).data?.find((d) => d.id === id);

export const RepoComponents: RequiredResourceComponents = {
  Name: ({ id }: { id: string }) => <>{useRepo(id)?.name}</>,
  Description: ({ id }) => <>{id}</>,
  Icon: () => <GitBranch className="w-4 h-4" />,
  Link: ({ id }) => <ResourceLink type="Repo" id={id} />,
  Info: [],
  Status: () => <></>,
  Actions: [PullRepo, CloneRepo],
  Page: {
    Config: RepoConfig,
    Danger: ({ id }) => (
      <Section
        title="Danger Zone"
        icon={<AlertTriangle className="w-4 h-4" />}
        actions={<CopyResource type="Repo" id={id} />}
      >
        <DeleteRepo id={id} />
      </Section>
    ),
  },
  Table: () => {
    const repos = useRead("ListRepos", {}).data;
    return (
      <DataTable
        data={repos ?? []}
        columns={[
          {
            accessorKey: "id",
            header: "Name",
            cell: ({ row }) => <RepoComponents.Link id={row.original.id} />,
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
