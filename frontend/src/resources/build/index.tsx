import { useSetRecentlyViewed } from "@hooks";
import { Resource } from "@layouts/resource";
import { BuildName, BuildVersion } from "./util";
import { Link, useParams } from "react-router-dom";
import { RebuildBuild } from "./components/actions";
import { Button } from "@ui/button";
import { Settings } from "lucide-react";
import { BuildConfig } from "./config";

export const BuildPage = () => {
  const { buildId } = useParams();
  const push = useSetRecentlyViewed();

  if (!buildId) return null;
  push("Build", buildId);

  return (
    <Resource
      title={<BuildName id={buildId} />}
      info={
        <div className="text-muted-foreground">
          <BuildVersion id={buildId} />
        </div>
      }
      actions={
        <div className="flex gap-4">
          <RebuildBuild buildId={buildId} />
          <Link to={`/builds/${buildId}/config`}>
            <Button variant="outline">
              <Settings className="w-4 h-4" />
            </Button>
          </Link>
        </div>
      }
    >
      <BuildConfig />
    </Resource>
  );
};
