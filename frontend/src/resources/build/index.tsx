import { useSetRecentlyViewed } from "@hooks";
import { Resource } from "@layouts/resource";
import { BuildName, BuildVersion } from "./util";
import { Link, useParams } from "react-router-dom";
import { RebuildBuild } from "./components/actions";
import { Button } from "@ui/button";
import { Settings } from "lucide-react";
import { BuildConfig } from "./config";

export const BuildPage = () => {
  const id = useParams().buildId;
  const push = useSetRecentlyViewed();

  if (!id) return null;
  push("Build", id);

  return (
    <Resource
      title={<BuildName id={id} />}
      info={
        <div className="text-muted-foreground">
          <BuildVersion id={id} />
        </div>
      }
      actions={
        <div className="flex gap-4">
          <RebuildBuild buildId={id} />
          <Link to={`/builds/${id}/config`}>
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
