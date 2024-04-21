import { Page, Section } from "@components/layouts";
import { Box, History, Key, Tag } from "lucide-react";
import { Link, useNavigate } from "react-router-dom";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@ui/card";
import { ResourceComponents } from "@components/resources";
import { OpenAlerts } from "@components/alert";
import { useRead, useUser } from "@lib/hooks";
import { ResourceLink } from "@components/resources/common";
import { Fragment } from "react";

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
        {recently_viewed?.slice(0, 6).map(({ type, id }) => (
          <Fragment key={type + id}>
            {type !== "System" && (
              <Card
                onClick={() => nav(`/${type.toLowerCase()}s/${id}`)}
                className="px-3 py-2 h-fit hover:bg-accent/50 group-focus:bg-accent/50 transition-colors cursor-pointer"
              >
                <CardContent className="flex items-center justify-between gap-4 px-3 py-2 text-sm text-muted-foreground">
                  <ResourceLink type={type} id={id} />
                  {type}
                </CardContent>
              </Card>
            )}
          </Fragment>
        ))}
      </div>
    </Section>
  );
};

const TagsSummary = () => {
  const tags_count = useRead("ListTags", {}).data?.length;

  return (
    <Link to="/tags" className="w-full">
      <Card className="hover:bg-accent/50 transition-colors cursor-pointer">
        <CardHeader>
          <div className="flex justify-between">
            <div>
              <CardTitle>Tags</CardTitle>
              <CardDescription>{tags_count} Total</CardDescription>
            </div>
            <Tag className="w-4 h-4" />
          </div>
        </CardHeader>
      </Card>
    </Link>
  );
};

const ApiKeysSummary = () => {
  const keys_count = useRead("ListApiKeys", {}).data?.length;

  return (
    <Link to="/keys" className="w-full">
      <Card className="hover:bg-accent/50 transition-colors cursor-pointer">
        <CardHeader>
          <div className="flex justify-between">
            <div>
              <CardTitle>Api Keys</CardTitle>
              <CardDescription>{keys_count} Total</CardDescription>
            </div>
            <Key className="w-4 h-4" />
          </div>
        </CardHeader>
      </Card>
    </Link>
  );
};
