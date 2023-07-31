import { useRead } from "@hooks";
import { Button } from "@ui/button";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
  CardDescription,
} from "@ui/card";
import { fmt_update_date } from "@util/helpers";
import { Bell, ExternalLink, User, Calendar } from "lucide-react";
import { Link } from "react-router-dom";
import { UpdateDetails } from "./details";

export const ResourceUpdates = ({ id }: { id: string }) => {
  const { data: updates, isLoading } = useRead("ListUpdates", {
    target: { id },
  });

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
        {isLoading && (
          <Card>
            <CardHeader>
              <CardTitle>...</CardTitle>
              <CardContent>
                <CardDescription className="flex items-center gap-2">
                  <User className="w-4 h-4" /> ...
                </CardDescription>
                <CardDescription className="flex items-center gap-2">
                  <Calendar className="w-4 h-4" /> ...
                </CardDescription>
              </CardContent>
            </CardHeader>
          </Card>
        )}
        {updates?.updates.slice(0, 3).map((update) => (
          <UpdateDetails update={update} key={update._id?.$oid}>
            <Card hoverable>
              <CardHeader>
                <CardTitle>{update.operation}</CardTitle>
              </CardHeader>
              <CardContent>
                <CardDescription className="flex items-center gap-2">
                  <User className="w-4 h-4" /> {update.operator}
                </CardDescription>
                <CardDescription className="flex items-center gap-2">
                  <Calendar className="w-4 h-4" />
                  {fmt_update_date(new Date(update.start_ts))}
                </CardDescription>
              </CardContent>
            </Card>
          </UpdateDetails>
        ))}
      </div>
    </div>
  );
};
