import { useRead } from "@lib/hooks";
import { Page, Section } from "@components/layouts";
import { Box } from "lucide-react";
import { DeploymentsChart } from "@components/dashboard/deployments-chart";
import { ServersChart } from "@components/dashboard/servers-chart";
import { BuildChart } from "@components/dashboard/builds-chart";

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

const MyResources = () => (
  <Section title="My Resources" icon={<Box className="w-4 h-4" />} actions="">
    <div className="flex flex-col lg:flex-row gap-4 w-full">
      <div className="flex flex-col md:flex-row gap-4 w-full">
        <ServersChart />
        <DeploymentsChart />
      </div>
      <div className="w-full lg:max-w-[50%]">
        <BuildChart />
      </div>
    </div>
  </Section>
);

export const Dashboard = () => {
  const user = useRead("GetUser", {}).data;

  return (
    <Page title={`Hello, ${user?.username}.`}>
      {/* <RecentlyViewed /> */}
      <MyResources />
    </Page>
  );
};
