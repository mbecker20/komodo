import { useParams } from "react-router-dom";
import { useRead, useSetRecentlyViewed } from "@hooks";
import { DeploymentLogs } from "./components/deployment-logs";
import { Card, CardHeader, CardTitle } from "@ui/card";
import { Bell } from "lucide-react";

const DeploymentUpdates = ({ id }: { id: string }) => {
  const updates = useRead("ListUpdates", { target: { id } }).data;

  return (
    <div className="flex flex-col">
      <div className="flex items-center gap-2 text-muted-foreground">
        <Bell className="w-4 h-4" />
        <h2 className="text-xl">Updates</h2>
      </div>
      <div className="grid md:grid-cols-3">
        {updates?.slice(0, 3).map((u) => (
          <Card>
            <CardTitle>
              <CardHeader>{u.operation}</CardHeader>
            </CardTitle>
          </Card>
        ))}
      </div>
    </div>
  );
};

export const DeploymentPage = () => {
  const { deploymentId } = useParams();
  const push = useSetRecentlyViewed();

  if (!deploymentId) return null;
  push("Deployment", deploymentId);

  return (
    <div className="flex flex-col gap-12">
      <DeploymentUpdates id={deploymentId} />
      <DeploymentLogs />
    </div>
  );
};
