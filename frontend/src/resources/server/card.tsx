import { useRead } from "@hooks";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@ui/card";
import { Link } from "react-router-dom";
import { ServerStatusIcon, ServerStats } from "./util";

export const ServerCard = ({ id }: { id: string }) => {
  const servers = useRead({ type: "ListServers", params: {} }).data;
  const server = servers?.find((server) => server.id === id);
  if (!server) return null;

  return (
    <Link to={`/servers/${server.id}`} key={server.id}>
      <Card hoverable>
        <CardHeader className="flex flex-row justify-between items-start">
          <div>
            <CardTitle>{server.name}</CardTitle>
            <CardDescription>{server.status}</CardDescription>
          </div>
          <ServerStatusIcon serverId={server.id} />
        </CardHeader>
        <CardContent>
          <ServerStats serverId={server.id} />
        </CardContent>
      </Card>
    </Link>
  );
};
