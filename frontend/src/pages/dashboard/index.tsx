import { useUser } from "@hooks";
import { Card, CardHeader, CardTitle } from "@ui/card";
import { ServersChart } from "./components/servers-chart";
import { DeploymentsChart } from "./components/deployments-chart";
import { Link } from "react-router-dom";
import { RecentlyViewed } from "./components/recently-viewed";
import { Box } from "lucide-react";

export const Dashboard = () => {
  const user = useUser().data;

  return (
    <div className="flex flex-col gap-12">
      <RecentlyViewed />
      <div className="flex flex-col gap-6 w-full">
        <div className="flex items-center gap-2">
          <Box className="w-6 h-6" />
          <h1 className="text-4xl"> Resources </h1>
        </div>
        <div className="flex gap-4">
          <div className="flex gap-4 w-full h-fit">
            <DeploymentsChart />
            <ServersChart />
          </div>
          <Link to="/builds" className="w-full max-w-[50%] h-full">
            <Card hoverable>
              <CardHeader>
                <CardTitle>Builds</CardTitle>
              </CardHeader>
            </Card>
          </Link>
        </div>
      </div>
    </div>
  );
};
