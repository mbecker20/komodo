import { useRead, useUser } from "@hooks";
import { Card, CardDescription, CardHeader, CardTitle } from "@ui/card";
import { version_to_string } from "@util/helpers";
import { ServersChart } from "./components/servers-chart";
import { DeploymentsChart } from "./components/deployments-chart";
import { Input } from "@ui/input";
import { Button } from "@ui/button";
import { PlusCircle } from "lucide-react";
import { Link } from "react-router-dom";

const DeploymentsList = () => {
  const deployments = useRead({ type: "ListDeployments", params: {} }).data;

  return (
    <div className="flex flex-col gap-2 w-full border-r pr-4">
      <h2 className="text-lg">Deployments</h2>
      {deployments?.map((deployment) => (
        <Card>
          <CardHeader>
            <CardTitle>{deployment.name}</CardTitle>
            <CardDescription>{deployment.version}</CardDescription>
          </CardHeader>
        </Card>
      ))}
    </div>
  );
};

const ServersList = () => {
  const servers = useRead({ type: "ListServers", params: {} }).data;

  return (
    <div className="flex flex-col gap-2 w-full border-r pr-4">
      <h2 className="text-lg">Servers</h2>
      {servers?.map((server) => (
        <Link to={`/servers/${server.id}`} key={server.id}>
          <Card>
            <CardHeader>
              <CardTitle>{server.name}</CardTitle>
              <CardDescription>{server.status}</CardDescription>
            </CardHeader>
          </Card>
        </Link>
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
          <Button className="w-[120px]" variant="outline" intent="success">
            <PlusCircle className="w-4 h-4 mr-2 text-green-500" />
            Add New
          </Button>
        </div>
      </div>
      <div className="flex gap-4">
        <DeploymentsChart />
        <ServersChart />
      </div>
      <div className="flex gap-4">
        <DeploymentsList />
        <ServersList />
        <BuildsList />
      </div>
    </>
  );
};
