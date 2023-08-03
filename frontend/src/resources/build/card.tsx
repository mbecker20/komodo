import { useRead } from "@hooks";
import { version_to_string } from "@util/helpers";
import { Link } from "react-router-dom";
import { BuildInfo } from "./util";
import { Hammer } from "lucide-react";
import { ResourceCard } from "@layouts/card";

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
