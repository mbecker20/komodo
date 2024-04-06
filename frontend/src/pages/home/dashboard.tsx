import { Page, Section } from "@components/layouts";
import { Box, FolderTree } from "lucide-react";
import { Link } from "react-router-dom";
import { Card, CardDescription, CardHeader, CardTitle } from "@ui/card";
import { TagsSummary } from "@components/dashboard/tags";
import { ApiKeysSummary } from "@components/dashboard/api-keys";
import { ResourceComponents } from "@components/resources";
import { OpenAlerts } from "@components/alert";

export const Dashboard = () => {
  return (
    <Page title="">
      {/* <RecentlyViewed /> */}
      <OpenAlerts />
      <Resources />
    </Page>
  );
};

const Resources = () => (
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
      <div className="w-full lg:max-w-[50%]">
        <Link to="/tree" className="w-full">
          <Card>
            <CardHeader className="justify-between">
              <div>
                <CardTitle>Tree</CardTitle>
                <CardDescription>
                  Visualize your servers / deployments
                </CardDescription>
              </div>
              <FolderTree className="w-4 h-4" />
            </CardHeader>
          </Card>
        </Link>
      </div>
      <div className="flex flex-col md:flex-row gap-4 w-full">
        <ResourceComponents.Repo.Dashboard />
        <ResourceComponents.Builder.Dashboard />
        {/* <TagsSummary />
        <ApiKeysSummary /> */}
      </div>
    </div>
    <div className="flex flex-col lg:flex-row gap-4 w-full">
      <div className="flex flex-col md:flex-row gap-4 w-full">
        <ResourceComponents.Alerter.Dashboard />
        <ResourceComponents.Procedure.Dashboard />
      </div>
      <div className="flex flex-col md:flex-row gap-4 w-full">
        <TagsSummary />
        <ApiKeysSummary />
      </div>
    </div>
  </Section>
);

// const RecentlyViewed = () => (
//   <Section
//     title="Recently Viewed"
//     icon={<History className="w-4 h-4" />}
//     actions=""
//   >
//     <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-4">
//       {useRead("GetUser", {})
//         .data?.recently_viewed?.slice(0, 6)
//         .map(
//           (target) =>
//             target.type !== "System" && (
//               <ResourceCard target={target} key={target.id} />
//             )
//         )}
//     </div>
//   </Section>
// );
