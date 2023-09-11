import { useRead } from "@lib/hooks";
import { Button } from "@ui/button";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
  CardDescription,
} from "@ui/card";
import { Bell, ExternalLink, User, Calendar, Check, X } from "lucide-react";
import { Link } from "react-router-dom";
import { Types } from "@monitor/client";
import { Section } from "@components/layouts";
import { fmt_update_date } from "@lib/utils";
import { UpdateDetails, UpdateUser } from "./details";

const UpdatePlaceHolder = () => (
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
);

const UpdateCard = ({ update }: { update: Types.UpdateListItem }) => (
  <UpdateDetails id={update.id}>
    <Card className="cursor-pointer hover:translate-y-[-2.5%] hover:bg-accent/50 transition-all">
      <CardHeader className="flex-row justify-between">
        <CardTitle>{update.operation}</CardTitle>
        {update.success ? (
          <Check className="w-4 h-4 stroke-green-500" />
        ) : (
          <X className="w-4 h-4 stroke-red-500" />
        )}
      </CardHeader>
      <CardContent>
        <CardDescription className="flex items-center gap-2">
          <User className="w-4 h-4" /> <UpdateUser user_id={update.operator} />
        </CardDescription>
        <CardDescription className="flex items-center gap-2">
          <Calendar className="w-4 h-4" />
          {fmt_update_date(new Date(update.start_ts))}
        </CardDescription>
      </CardContent>
    </Card>
  </UpdateDetails>
);

export const ResourceUpdates = ({ type, id }: Types.ResourceTarget) => {
  const { data, isLoading } = useRead("ListUpdates", {
    query: {
      "target.type": type,
      "target.id": id,
    },
  });

  return (
    <Section
      title="Updates"
      icon={<Bell className="w-4 h-4" />}
      actions={
        <Link to={`/deployments/${id}/updates`}>
          <Button variant="secondary">
            <ExternalLink className="w-4 h-4" />
          </Button>
        </Link>
      }
    >
      <div className="grid md:grid-cols-3 gap-4">
        {isLoading && <UpdatePlaceHolder />}
        {data?.updates
          .slice(0, 3)
          .map((update) => <UpdateCard update={update} key={update.id} />)}
      </div>
    </Section>
  );
};
