import { useAddRecentlyViewed, useWrite } from "@hooks";
import { Resource } from "@layouts/resource";
import { BuildName, BuildVersion } from "./util";
import { Link, useParams } from "react-router-dom";
import { RebuildBuild } from "./components/actions";
import { useRead } from "@hooks";
import { version_to_string } from "@util/helpers";
import { BuildInfo } from "./util";
import { Hammer } from "lucide-react";
import { ResourceCard } from "@layouts/card";
import { ResourceUpdates } from "@components/updates/resource";
import { Types } from "@monitor/client";
import { useState } from "react";
import { ConfigLayout } from "@layouts/page";
import { Button } from "@ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@ui/card";
import { ConfigAgain } from "@components/config/again";
import { BuilderSelector } from "./config";

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

const BuildConfigInner = ({
  id,
  config,
}: {
  id: string;
  config: Types.BuildConfig;
}) => {
  const [update, set] = useState<Partial<Types.BuildConfig>>({});
  const [show, setShow] = useState("general");
  const { mutate } = useWrite("UpdateBuild");

  return (
    <ConfigLayout
      content={update}
      onConfirm={() => mutate({ id, config: update })}
      onReset={() => set({})}
    >
      <div className="flex gap-4">
        <div className="flex flex-col gap-4 w-[300px]">
          {["general", "repo", "docker", "volumes"].map((item) => (
            <Button
              variant={show === item ? "secondary" : "outline"}
              onClick={() => setShow(item)}
              className="capitalize"
            >
              {item}
            </Button>
          ))}
        </div>
        <Card className="w-full">
          <CardHeader className="border-b">
            <CardTitle className="capitalize">{show}</CardTitle>
          </CardHeader>
          <CardContent className="flex flex-col gap-4 mt-4">
            {/* General Config */}
            {show === "general" && (
              <ConfigAgain
                config={config}
                update={update}
                set={(u) => set((p) => ({ ...p, ...u }))}
                components={{
                  builder_id: (id, set) => (
                    <BuilderSelector
                      selected={id}
                      onSelect={(builder_id) => set({ builder_id })}
                    />
                  ),
                }}
              />
            )}

            {/* Networking Config */}
            {show === "repo" && (
              <ConfigAgain
                config={config}
                update={update}
                set={(u) => set((p) => ({ ...p, ...u }))}
                components={{ repo: true, branch: true, github_account: true }}
              />
            )}

            {/* Environment Config */}
            {show === "docker" && (
              <ConfigAgain
                config={config}
                update={update}
                set={(u) => set((p) => ({ ...p, ...u }))}
                components={{
                  build_path: true,
                  dockerfile_path: true,
                  docker_account: true,
                  // docker_organization,
                  use_buildx: true,
                }}
              />
            )}

            {/* Environment Config
            {show === "volumes" && (
              <ConfigAgain
                config={config}
                update={update}
                set={(u) => set((p) => ({ ...p, ...u }))}
                components={{
                  volumes: (value, set) => (
                    <PortsConfig ports={value} set={set} />
                  ),
                }}
              />
            )} */}
          </CardContent>
        </Card>
      </div>
    </ConfigLayout>
  );
};

const BuildConifig = ({ id }: { id: string }) => {
  const config = useRead("GetBuild", { id }).data?.config;
  if (!config) return null;
  return <BuildConfigInner id={id} config={config} />;
};

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
      <BuildConifig id={id} />
    </Resource>
  );
};
