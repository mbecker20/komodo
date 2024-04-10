import { Page, Section } from "@components/layouts";
import { Box, History } from "lucide-react";
import { useNavigate } from "react-router-dom";
import { Card, CardContent } from "@ui/card";
import { TagsSummary } from "@components/dashboard/tags";
import { ApiKeysSummary } from "@components/dashboard/api-keys";
import { ResourceComponents } from "@components/resources";
import { OpenAlerts } from "@components/alert";
import { useUser } from "@lib/hooks";
import { ResourceLink } from "@components/resources/common";

export const Dashboard = () => {
  return (
    <Page title="">
      <OpenAlerts />
      <RecentlyViewed />
      <Resources />
    </Page>
  );
};

const Resources = () => {
  return (
    <Section title="Resources" icon={<Box className="w-4 h-4" />} actions="">
      <div className="flex flex-col lg:flex-row gap-4 w-full">
        <div className="flex flex-col md:flex-row gap-4 w-full">
          <ResourceComponents.Server.Dashboard />
          <ResourceComponents.Deployment.Dashboard />
        </div>
        <div className="w-full lg:max-w-[50%]">
          <ResourceComponents.Build.Dashboard />
        </div>
      </div>
      <div className="flex flex-col lg:flex-row gap-4 w-full">
        <ResourceComponents.Procedure.Dashboard />
        <ResourceComponents.Repo.Dashboard />
      </div>
      <div className="flex flex-col lg:flex-row gap-4 w-full">
        <div className="flex flex-col md:flex-row gap-4 w-full">
          <ResourceComponents.Alerter.Dashboard />
          <ResourceComponents.Builder.Dashboard />
        </div>
        <div className="flex flex-col md:flex-row gap-4 w-full">
          <TagsSummary />
          <ApiKeysSummary />
        </div>
      </div>
    </Section>
  );
};

const RecentlyViewed = () => {
  const nav = useNavigate();
  const recently_viewed = useUser().data?.recently_viewed;
  return (
    <Section
      title="Recently Viewed"
      icon={<History className="w-4 h-4" />}
      actions=""
    >
      <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-4">
        {recently_viewed?.slice(0, 6).map(
          ({ type, id }) =>
            type !== "System" && (
              <Card
                onClick={() => nav(`/${type.toLowerCase()}s/${id}`)}
                className="px-3 py-2 h-fit hover:bg-accent/50 group-focus:bg-accent/50 transition-colors cursor-pointer"
              >
                <CardContent className="flex items-center justify-between gap-4 px-3 py-2 text-sm text-muted-foreground">
                  <ResourceLink type={type} id={id} />
                  {type}
                </CardContent>
              </Card>
            )
        )}
      </div>
    </Section>
  );
};
