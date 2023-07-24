import { Resource } from "@layouts/resource";
import { useParams } from "react-router-dom";
import { useExecute, useRead, useSetRecentlyViewed } from "@hooks";
import { ActionButton } from "@components/util";
import { Hammer } from "lucide-react";
import { version_to_string } from "@util/helpers";

export const BuildName = ({ id }: { id: string }) => {
  const builds = useRead({ type: "ListBuilds", params: {} }).data;
  const build = builds?.find((b) => b.id === id);
  return <>{build?.name ?? "..."}</>;
};

export const BuildVersion = ({ id }: { id: string }) => {
  const builds = useRead({ type: "ListBuilds", params: {} }).data;
  const build = builds?.find((b) => b.id === id);
  return <>{version_to_string(build?.version) ?? "..."}</>;
};

export const RebuildBuild = ({ buildId }: { buildId: string }) => {
  const { mutate, isLoading } = useExecute();
  return (
    <ActionButton
      title="Build"
      intent="success"
      icon={<Hammer className="h-4 w-4" />}
      onClick={() =>
        mutate({ type: "RunBuild", params: { build_id: buildId } })
      }
      disabled={isLoading}
    />
  );
};

export const Build = () => {
  const { buildId } = useParams();
  const push = useSetRecentlyViewed();

  if (!buildId) return null;
  push("Build", buildId);

  return (
    <Resource
      title={<BuildName id={buildId} />}
      info={<BuildVersion id={buildId} />}
      actions={<RebuildBuild buildId={buildId} />}
      tabs={[
        {
          title: "Config",
          component: "config",
        },
        {
          title: "Builder",
          component: "builder",
        },
        {
          title: "Updates",
          component: "updates",
        },
      ]}
    />
  );
};
