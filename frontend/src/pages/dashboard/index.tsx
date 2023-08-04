import { ServersChart } from "./components/servers-chart";
import { DeploymentsChart } from "./components/deployments-chart";
import { RecentlyViewed } from "./components/recently-viewed";
import { Box } from "lucide-react";
import { BuildChart } from "./components/builds-chart";
import { Page, Section } from "@layouts/page";
import { useUser } from "@hooks";
import { ResourceOverviewCard } from "@layouts/card";
import { CreateResource } from "./components/create-resource";

const DashboardTitle = () => {
  const user = useUser().data;
  return <>Hello, {user?.username}.</>;
};

const MyResources = () => (
  <Section title="My Resources" icon={<Box className="w-4 h-4" />} actions="">
    <div className="flex flex-col gap-4">
      <div className="flex flex-col lg:flex-row gap-4">
        <div className="flex flex-col md:flex-row gap-4 w-full">
          <ResourceOverviewCard type="Server">
            <ServersChart />
          </ResourceOverviewCard>
          <ResourceOverviewCard type="Deployment">
            <DeploymentsChart />
          </ResourceOverviewCard>
        </div>
        <div className="w-full lg:max-w-[50%]">
          <ResourceOverviewCard type="Build">
            <BuildChart />
          </ResourceOverviewCard>
        </div>
      </div>
      <div className="flex gap-4">
        <ResourceOverviewCard type="Builder" />
        <ResourceOverviewCard type="Alerter" />
        <ResourceOverviewCard type="Repo" />
      </div>
    </div>
  </Section>
);

export const Dashboard = () => (
  <Page title={<DashboardTitle />} subtitle="" actions={<CreateResource />}>
    <RecentlyViewed />
    <MyResources />
  </Page>
);
