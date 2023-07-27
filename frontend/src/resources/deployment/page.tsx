import { Link, useParams } from "react-router-dom";
import { useRead, useSetRecentlyViewed } from "@hooks";
import { DeploymentLogs } from "./components/deployment-logs";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@ui/card";
import { Bell, Calendar, ExternalLink, User } from "lucide-react";
import { fmt_update_date } from "@util/helpers";
import { Button } from "@ui/button";

const DeploymentUpdates = ({ id }: { id: string }) => {
  const updates = useRead("ListUpdates", { target: { id } }).data;

  return (
    <div className="flex flex-col">
      <div className="flex justify-between">
        <div className="flex items-center gap-2 text-muted-foreground">
          <Bell className="w-4 h-4" />
          <h2 className="text-xl">Updates</h2>
        </div>
        <Link to={`/deployments/${id}/updates`}>
          <Button variant="secondary">
            <ExternalLink className="w-4 h-4" />
          </Button>
        </Link>
      </div>
      <div className="grid md:grid-cols-3 mt-2 gap-4">
        {updates?.slice(0, 3).map((update) => (
          <Card>
            <CardHeader>
              <CardTitle>{update.operation}</CardTitle>
            </CardHeader>
            <CardContent>
              <CardDescription className="flex items-center gap-2">
                <Calendar className="w-4 h-4" />
                {fmt_update_date(new Date(update.start_ts))}
              </CardDescription>
              <CardDescription className="flex items-center gap-2">
                <User className="w-4 h-4" /> {update.operator}
              </CardDescription>
            </CardContent>
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
