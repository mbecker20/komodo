import { useRead, useUser, useWrite } from "@hooks";
import { Card, CardDescription, CardHeader, CardTitle } from "@ui/card";
import { version_to_string } from "@util/helpers";
import { ServersChart } from "./components/servers-chart";
import { DeploymentsChart } from "./components/deployments-chart";
import { Input } from "@ui/input";
import { ChevronDown, PlusCircle } from "lucide-react";
import { Link } from "react-router-dom";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@ui/dropdown";
import { Button } from "@ui/button";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@ui/dialog";
import { useState } from "react";
import { RecentlyViewed } from "./components/recents";

const NewDeployment = ({
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
          <DialogTitle>New Deployment</DialogTitle>
        </DialogHeader>
        <div className="flex items-center justify-between">
          <div>Deployment Name</div>
          <Input
            className="max-w-[50%]"
            placeholder="Deployment Name"
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
                type: "CreateDeployment",
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

const NewButton = () => {
  const [open, set] = useState<"deployment" | "server" | boolean>(false);
  return (
    <>
      <DropdownMenu>
        <DropdownMenuTrigger>
          <Button
            className="w-[200px] flex items-center justify-between"
            variant="outline"
          >
            <div className="flex items-center gap-2">
              <PlusCircle className="w-4 h-4 text-green-500" />
              Add New
            </div>
            <ChevronDown className="w-4 h-4" />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent className="w-[200px]">
          <DropdownMenuLabel className="text-xs">
            Resource Type
          </DropdownMenuLabel>
          <DropdownMenuSeparator />
          <DropdownMenuGroup>
            <DropdownMenuItem onClick={() => set("deployment")}>
              Deployment
            </DropdownMenuItem>
            <DropdownMenuItem> Build </DropdownMenuItem>
            <DropdownMenuItem> Server </DropdownMenuItem>
          </DropdownMenuGroup>
        </DropdownMenuContent>
      </DropdownMenu>
      <NewDeployment open={open === "deployment"} set={set} />
    </>
  );
};

export const DeploymentCard = ({ id }: { id: string }) => {
  const deployments = useRead({ type: "ListDeployments", params: {} }).data;
  const deployment = deployments?.find((d) => d.id === id);
  if (!deployment) return null;
  return (
    <Link to={`/deployments/${deployment.id}`}>
      <Card className="hover:bg-accent">
        <CardHeader>
          <CardTitle>{deployment.name}</CardTitle>
          <CardDescription>
            {deployment.status ?? "not deployed"}
          </CardDescription>
        </CardHeader>
      </Card>
    </Link>
  );
};

const DeploymentsList = () => {
  const deployments = useRead({ type: "ListDeployments", params: {} }).data;

  return (
    <div className="flex flex-col gap-2 w-full border-r pr-4">
      <h2 className="text-lg">Deployments</h2>
      {deployments?.map(({ id }) => (
        <DeploymentCard key={id} id={id} />
      ))}
    </div>
  );
};

export const ServerCard = ({ id }: { id: string }) => {
  const servers = useRead({ type: "ListServers", params: {} }).data;
  const server = servers?.find((server) => server.id === id);
  if (!server) return null;

  return (
    <Link to={`/servers/${server.id}`} key={server.id}>
      <Card className="hover:bg-accent">
        <CardHeader>
          <CardTitle>{server.name}</CardTitle>
          <CardDescription>{server.status}</CardDescription>
        </CardHeader>
      </Card>
    </Link>
  );
};

const ServersList = () => {
  const servers = useRead({ type: "ListServers", params: {} }).data;

  return (
    <div className="flex flex-col gap-2 w-full border-r pr-4">
      <h2 className="text-lg">Deployments</h2>
      {servers?.map((server) => (
        <ServerCard key={server.id} id={server.id} />
      ))}
    </div>
  );
};

const BuildsList = () => {
  const builds = useRead({ type: "ListBuilds", params: {} }).data;

  return (
    <div className="flex flex-col gap-2 w-full">
      <h2 className="text-lg">Builds</h2>
      {builds?.map((build) => (
        <Card>
          <CardHeader key={build.id}>
            <CardTitle>{build.name}</CardTitle>
            <CardDescription>
              {version_to_string(build.version)}
            </CardDescription>
          </CardHeader>
        </Card>
      ))}
    </div>
  );
};

export const Dashboard = () => {
  const user = useUser().data;

  return (
    <>
      <div className="flex items-center justify-between">
        <h1 className="text-xl"> Hello, {user?.username}.</h1>
        <div className="flex gap-4">
          <Input className="w-[300px]" placeholder="Search" />
          <NewButton />
        </div>
      </div>
      <div className="flex gap-24">
        <div className="flex  gap-4 w-full h-fit">
          <DeploymentsChart />
          <ServersChart />
        </div>
        <RecentlyViewed />
      </div>
      {/* <div className="flex gap-4">
        <DeploymentsList />
        <ServersList />
        <BuildsList />
      </div> */}
    </>
  );
};
