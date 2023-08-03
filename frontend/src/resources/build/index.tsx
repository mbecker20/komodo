import { useWrite } from "@hooks";
import { Resource } from "@layouts/resource";
import { BuildName, BuildVersion } from "./util";
import { Link, useParams } from "react-router-dom";
import { RebuildBuild } from "./components/actions";
import { BuildConfig } from "./config";
import { useEffect } from "react";
import { useRead } from "@hooks";
import { version_to_string } from "@util/helpers";
import { BuildInfo } from "./util";
import { Hammer } from "lucide-react";
import { ResourceCard } from "@layouts/card";

export const BuildPage = () => {
  const id = useParams().buildId;
  const push = useWrite("PushRecentlyViewed").mutate;

  if (!id) return null;
  useEffect(() => {
    push({ resource: { type: "Build", id } });
  }, []);

  return (
    <Resource
      title={<BuildName id={id} />}
      info={
        <div className="text-muted-foreground">
          <BuildVersion id={id} />
        </div>
      }
      actions={<RebuildBuild buildId={id} />}
    >
      <BuildConfig />
    </Resource>
  );
};

export const BuildCard = ({ id }: { id: string }) => {
  const builds = useRead("ListBuilds", {}).data;
  const build = builds?.find((server) => server.id === id);
  if (!build) return null;

  return (
    <Link to={`/builds/${build.id}`} key={build.id}>
      <ResourceCard
        title={build.name}
        description={version_to_string(build.version) ?? "not built"}
        statusIcon={<Hammer className="w-4 h-4" />}
      >
        <BuildInfo id={id} />
      </ResourceCard>
    </Link>
  );
};
