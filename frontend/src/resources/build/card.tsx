import { useRead } from "@hooks";
import { ServerStatusIcon } from "@resources/server/util";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@ui/card";
import { version_to_string } from "@util/helpers";
import { Link } from "react-router-dom";
import { BuildInfo } from "./util";
import { Hammer } from "lucide-react";

export const BuildCard = ({ id }: { id: string }) => {
  const builds = useRead("ListBuilds", {}).data;
  const build = builds?.find((server) => server.id === id);
  if (!build) return null;

  return (
    <Link to={`/builds/${build.id}`} key={build.id}>
      <Card hoverable>
        <CardHeader className="flex flex-row justify-between">
          <div>
            <CardTitle>{build.name}</CardTitle>
            <CardDescription>
              {version_to_string(build.version)}
            </CardDescription>
          </div>
          <ServerStatusIcon serverId={build.id} />
        </CardHeader>
        <CardContent className="flex items-center  gap-4">
          <Hammer className="w-4 h-4" />
          <div className="border h-6" />
          <BuildInfo id={id} />
        </CardContent>
      </Card>
    </Link>
  );
};
