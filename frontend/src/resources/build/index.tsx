import { useAddRecentlyViewed } from "@hooks";
import { Resource } from "@layouts/resource";
import { BuildName, BuildVersion } from "./util";
import { Link, useParams } from "react-router-dom";
import { RebuildBuild } from "./components/actions";
import { BuildConfig } from "./config";
import { useRead } from "@hooks";
import { version_to_string } from "@util/helpers";
import { BuildInfo } from "./util";
import { Hammer } from "lucide-react";
import { ResourceCard } from "@layouts/card";
import { ResourceUpdates } from "@components/updates/resource";

export const BuildPage = () => {
  const id = useParams().buildId;
  if (!id) return null;
  useAddRecentlyViewed("Build", id);

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
      <ResourceUpdates type="Build" id={id} />
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
        description={version_to_string(build.info.version) ?? "not built"}
        statusIcon={<Hammer className="w-4 h-4" />}
      >
        <BuildInfo id={id} />
      </ResourceCard>
    </Link>
  );
};
