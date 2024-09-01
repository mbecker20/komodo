import { useRead } from "@lib/hooks";
import { Button } from "@ui/button";
import {
  Bell,
  ExternalLink,
  Calendar,
  Check,
  X,
  Loader2,
  Milestone,
} from "lucide-react";
import { Link } from "react-router-dom";
import { Types } from "@komodo/client";
import { Section } from "@components/layouts";
import { UpdateDetails, UpdateUser } from "./details";
import { UpdateStatus } from "@komodo/client/dist/types";
import { fmt_date, fmt_operation, fmt_version } from "@lib/formatting";
import {
  getUpdateQuery,
  usableResourcePath,
  version_is_none,
} from "@lib/utils";
import { Card } from "@ui/card";
import { UsableResource } from "@types";

const UpdateCard = ({
  update,
  smallHidden,
}: {
  update: Types.UpdateListItem;
  smallHidden?: boolean;
}) => {
  const Icon = () => {
    if (update.status === UpdateStatus.Complete) {
      if (update.success) return <Check className="w-4 stroke-green-500" />;
      else return <X className="w-4 stroke-red-500" />;
    } else return <Loader2 className="w-4 animate-spin" />;
  };

  return (
    <UpdateDetails id={update.id}>
      <Card
        className={`p-4 ${
          smallHidden ? "hidden xl:" : ""
        }flex justify-between cursor-pointer hover:bg-accent/50 transition-colors text-sm`}
      >
        <div className="grid gap-1 items-start">
          <div className="flex items-center gap-2">
            <Icon />
            {fmt_operation(update.operation)}
          </div>
          {!version_is_none(update.version) && (
            <div className="flex items-center gap-2 text-muted-foreground">
              <Milestone className="w-4" />
              {fmt_version(update.version)}
            </div>
          )}
        </div>
        <div className="grid gap-1">
          <div className="flex items-center gap-2 text-muted-foreground">
            <Calendar className="w-4" />
            {fmt_date(new Date(update.start_ts))}
          </div>
          <UpdateUser user_id={update.operator} muted />
        </div>
      </Card>
    </UpdateDetails>
  );
};

export const AllUpdates = () => {
  const updates = useRead("ListUpdates", {}).data;

  return (
    <Section
      title="Updates"
      icon={<Bell className="w-4 h-4" />}
      actions={
        <Link to="/updates">
          <Button variant="secondary" size="icon">
            <ExternalLink className="w-4 h-4" />
          </Button>
        </Link>
      }
    >
      <div className="grid md:grid-cols-2 xl:grid-cols-3 gap-4">
        {updates?.updates.slice(0, 3).map((update, i) => (
          <UpdateCard update={update} key={update.id} smallHidden={i > 1} />
        ))}
      </div>
    </Section>
  );
};

export const ResourceUpdates = ({ type, id }: Types.ResourceTarget) => {
  const deployments = useRead("ListDeployments", {}).data;

  const updates = useRead("ListUpdates", {
    query: getUpdateQuery({ type, id }, deployments),
  }).data;

  return (
    <Section
      title="Updates"
      icon={<Bell className="w-4 h-4" />}
      actions={
        <Link
          to={`/${usableResourcePath(type as UsableResource)}/${id}/updates`}
        >
          <Button variant="secondary" size="icon">
            <ExternalLink className="w-4 h-4" />
          </Button>
        </Link>
      }
    >
      <div className="grid md:grid-cols-2 xl:grid-cols-3 gap-4">
        {updates?.updates.slice(0, 3).map((update, i) => (
          <UpdateCard update={update} key={update.id} smallHidden={i > 1} />
        ))}
      </div>
    </Section>
  );
};
