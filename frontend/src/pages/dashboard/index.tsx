import { useRead, useUser, useWrite } from "@hooks";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@ui/card";
import { version_to_string } from "@util/helpers";
import { ServersChart } from "./components/servers-chart";
import { DeploymentsChart } from "./components/deployments-chart";
import { Input } from "@ui/input";
import { Link } from "react-router-dom";
import { Button } from "@ui/button";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@ui/dialog";
import { useState } from "react";
import { RecentlyViewed } from "./components/recently-viewed";
import { ServerStats, ServerStatusIcon } from "@resources/server/util";

const NewBuild = ({
  open,
  set,
}: {
  open: boolean;
  set: (b: boolean) => void;
}) => {
  const { mutate } = useWrite();
  const [name, setName] = useState("");

  return (
    <Dialog open={open} onOpenChange={set}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>New Build</DialogTitle>
        </DialogHeader>
        <div className="flex items-center justify-between">
          <div>Build Name</div>
          <Input
            className="max-w-[50%]"
            placeholder="Build Name"
            name={name}
            onChange={(e) => setName(e.target.value)}
          />
        </div>
        <DialogFooter>
          <Button
            variant="outline"
            intent="success"
            onClick={() => {
              mutate({
                type: "CreateBuild",
                params: { name, config: {} },
              });
              set(false);
            }}
          >
            Create
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

export const ServerCard = ({ id }: { id: string }) => {
  const servers = useRead({ type: "ListServers", params: {} }).data;
  const server = servers?.find((server) => server.id === id);
  if (!server) return null;

  return (
    <Link to={`/servers/${server.id}`} key={server.id}>
      <Card hoverable>
        <CardHeader className="flex flex-row justify-between">
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

export const BuildCard = ({ id }: { id: string }) => {
  const builds = useRead({ type: "ListBuilds", params: {} }).data;
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
      </Card>
    </Link>
  );
};

export const Dashboard = () => {
  const user = useUser().data;

  return (
    <div className="flex gap-24">
      <div className="flex flex-col gap-6 w-full">
        <h1 className="text-3xl"> Hello, {user?.username}.</h1>
        <div className="flex flex-col gap-4">
          <div className="flex gap-4 w-full h-fit">
            <DeploymentsChart />
            <ServersChart />
          </div>
          <Link to="/builds">
            <Card hoverable>
              <CardHeader>
                <CardTitle>Builds</CardTitle>
              </CardHeader>
            </Card>
          </Link>
        </div>
      </div>
      <RecentlyViewed />
    </div>
  );
};
