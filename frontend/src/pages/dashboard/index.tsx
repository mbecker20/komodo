import { useRead, useUser } from "@hooks";
import { Card, CardDescription, CardHeader, CardTitle } from "@ui/card";
import { version_to_string } from "@util/helpers";
import { ServersChart } from "./components/servers-chart";
import { DeploymentsChart } from "./components/deployments-chart";
import { Link } from "react-router-dom";
import { RecentlyViewed } from "./components/recently-viewed";
import { ServerStatusIcon } from "@resources/server/util";

export const Dashboard = () => {
  const user = useUser().data;

  return (
    <div className="flex gap-24">
      <div className="flex flex-col gap-6 w-full">
        <h1 className="text-4xl"> Hello, {user?.username}.</h1>
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
