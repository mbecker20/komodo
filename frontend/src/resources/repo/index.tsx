import { ResourceUpdates } from "@components/updates/resource";
import { useAddRecentlyViewed, useRead } from "@hooks";
import { ResourceCard } from "@layouts/card";
import { Page } from "@layouts/page";
import { GitBranch } from "lucide-react";
import { useParams } from "react-router-dom";

const RepoName = ({ id }: { id: string }) => {
  const repos = useRead("ListRepos", {}).data;
  const repo = repos?.find((r) => r.id === id);
  if (!repo) return null;
  return <>{repo.name}</>;
};

export const RepoPage = () => {
  const id = useParams().repoId;
  if (!id) return null;
  useAddRecentlyViewed("Repo", id);

  return (
    <Page title="Repo" subtitle="" actions="">
      <ResourceUpdates type="Repo" id={id} />
      <RepoCard id={id} />
    </Page>
  );
};

export const RepoCard = ({ id }: { id: string }) => {
  const repos = useRead("ListRepos", {}).data;
  const repo = repos?.find((r) => r.id === id);
  if (!repo) return null;

  return (
    <ResourceCard title={repo.name} statusIcon={<GitBranch />} description="">
      <div></div>
    </ResourceCard>
  );
};
