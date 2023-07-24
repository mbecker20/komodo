import { useGetRecentlyViewed } from "@hooks";
import { Card, CardContent, CardHeader, CardTitle } from "@ui/card";
import { DeploymentCard, ServerCard } from "..";

export const RecentlyViewed = () => {
  const recents = useGetRecentlyViewed();
  return (
    <Card className="w-full">
      <CardHeader>
        <CardTitle>Recently Viewed</CardTitle>
      </CardHeader>
      <CardContent className="flex flex-col gap-4">
        {recents.map(({ type, id }) => {
          if (type === "Deployment") return <DeploymentCard key={id} id={id} />;
          if (type === "Build") return <div></div>;
          if (type === "Server") return <ServerCard key={id} id={id} />;
        })}
      </CardContent>
    </Card>
  );
};
