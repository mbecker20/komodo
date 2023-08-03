import { Card, CardContent, CardHeader, CardTitle } from "@ui/card";
import { ServersChart } from "./components/servers-chart";
import { DeploymentsChart } from "./components/deployments-chart";
import { Link } from "react-router-dom";
import { RecentlyViewed } from "./components/recently-viewed";
import { Box } from "lucide-react";
import { BuildChart } from "./components/builds-chart";
import { Page, Section } from "@layouts/page";
import { useUser } from "@hooks";

const DashboardTitle = () => {
  const user = useUser().data;
  return <>Hello, {user?.username}.</>;
};

const Builds = () => (
  <Link to="/builds" className="w-full">
    <Card hoverable>
      <CardHeader>
        <CardTitle>Builds</CardTitle>
      </CardHeader>
      <CardContent className="h-[200px]">
        <BuildChart />
      </CardContent>
    </Card>
  </Link>
);

const MyResources = () => (
  <Section title="My Resources" icon={<Box className="w-4 h-4" />} actions="">
    <div className="flex flex-col gap-4">
      <div className="flex flex-col lg:flex-row gap-4">
        <div className="flex flex-col md:flex-row gap-4 w-full">
          <DeploymentsChart />
          <ServersChart />
        </div>
        <Builds />
      </div>
      <div className="flex gap-4">
        <Link to="/builders" className="w-full h-full">
          <Card hoverable>
            <CardHeader>
              <CardTitle>Builders</CardTitle>
            </CardHeader>
          </Card>
        </Link>
        <Link to="/alerters" className="w-full h-full">
          <Card hoverable>
            <CardHeader>
              <CardTitle>alerters</CardTitle>
            </CardHeader>
          </Card>
        </Link>
        <Link to="/repos" className="w-full h-full">
          <Card hoverable>
            <CardHeader>
              <CardTitle>repos</CardTitle>
            </CardHeader>
          </Card>
        </Link>
      </div>
    </div>
  </Section>
);

export const Dashboard = () => (
  <Page title={<DashboardTitle />} subtitle="" actions="">
    <RecentlyViewed />
    <MyResources />
  </Page>
);
