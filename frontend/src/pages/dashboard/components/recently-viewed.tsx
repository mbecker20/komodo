import { useGetRecentlyViewed } from "@hooks";
import { Card, CardContent, CardHeader, CardTitle } from "@ui/card";
import { BuildCard, DeploymentCard, ServerCard } from "..";

export const RecentlyViewed = () => {
  const recents = useGetRecentlyViewed();
  return (
    <div className="w-full flex flex-col gap-6">
      <h2 className="text-xl">Recently Viewed</h2>
      <div className="flex flex-col gap-4">
        {recents.map(({ type, id }) => {
          if (type === "Deployment") return <DeploymentCard key={id} id={id} />;
          if (type === "Build") return <BuildCard key={id} id={id} />;
          if (type === "Server") return <ServerCard key={id} id={id} />;
        })}
      </div>
    </div>
  );
};
