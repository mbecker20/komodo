import { UpdateDetails } from "@components/updates/details";
import { Types } from "@komodo/client";
import {
  ColorIntention,
  text_color_class_by_intention,
  hex_color_by_intention,
} from "@lib/color";
import { fmt_operation, fmt_version, fmt_date } from "@lib/formatting";
import { useRead } from "@lib/hooks";
import {
  getUpdateQuery,
  usableResourcePath,
  cn,
  version_is_none,
} from "@lib/utils";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@radix-ui/react-tabs";
import { UsableResource } from "@types";
import {
  ExternalLink,
  Check,
  X,
  Loader2,
  Milestone,
  Calendar,
} from "lucide-react";
import { Link } from "react-router-dom";

export const ResourceNoficiations = ({ type, id }: Types.ResourceTarget) => {
  const deployments = useRead("ListDeployments", {}).data;

  const updates = useRead("ListUpdates", {
    query: getUpdateQuery({ type, id }, deployments),
  }).data;

  const alerts = useRead("ListAlerts", {
    query: getUpdateQuery({ type, id }, deployments),
  }).data;

  return (
    <div className="shrink-0 p-6 pt-5 pr-3 border rounded-md w-full xl:max-w-[500px]">
      <Tabs defaultValue={alerts?.alerts.length ? "alerts" : "updates"}>
        <TabsList>
          <TabsTrigger value="updates">Updates</TabsTrigger>
          <TabsTrigger value="alerts">Alerts</TabsTrigger>
        </TabsList>
        <div className="mt-4 pr-3 h-[150px] overflow-y-scroll">
          <TabsContent value="updates">
            {updates?.updates.slice(0, 10).map((update) => (
              <Update key={update.id} update={update} />
            ))}
            <ShowAll
              to={`/${usableResourcePath(
                type as UsableResource
              )}/${id}/updates`}
            />
          </TabsContent>
          <TabsContent value="alerts">
            {alerts?.alerts.some((alert) => !alert.resolved) ? (
              alerts?.alerts.slice(0, 10).map((alert) => "ALERT")
            ) : (
              <p className="pl-2 text-sm text-muted-foreground">
                No open alerts
              </p>
            )}
            <ShowAll
              to={`/${usableResourcePath(type as UsableResource)}/${id}/alerts`}
            />
          </TabsContent>
        </div>
      </Tabs>
    </div>
  );
};

const ShowAll = ({ to }: { to: string }) => (
  <Link
    to={to}
    className="mt-2 p-2 border rounded-md flex items-center justify-center text-muted-foreground border-dashed"
  >
    Show All <ExternalLink className="w-4 ml-4" />
  </Link>
);

const Update = ({ update }: { update: Types.UpdateListItem }) => {
  const intent: ColorIntention =
    update.status === Types.UpdateStatus.Complete
      ? update.success
        ? "Good"
        : "Critical"
      : "None";

  const color = text_color_class_by_intention(intent);
  const background = hex_color_by_intention(intent) + "25";

  const Icon = () =>
    update.status === Types.UpdateStatus.Complete ? (
      update.success ? (
        <Check className={cn("w-4", color)} />
      ) : (
        <X className={cn("w-4", color)} />
      )
    ) : (
      <Loader2 className={cn("w-4 animate-spin", color)} />
    );

  return (
    <UpdateDetails id={update.id}>
      <div
        className={
          "p-2 grid grid-cols-3 text-sm cursor-pointer odd:bg-accent/50 hover:bg-accent transition-all first:rounded-t-md last:rounded-b-md"
        }
      >
        <div className="flex items-center gap-2">
          <Icon />
          <p className={cn("font-bold", color)}>
            {fmt_operation(update.operation)}
          </p>
        </div>
        {!version_is_none(update.version) && (
          <div className="flex items-center gap-2">
            <Milestone className="w-4" />
            <p>{fmt_version(update.version)}</p>
          </div>
        )}
        <div className="flex items-center gap-2">
          <Calendar className="w-4" />
          {fmt_date(new Date(update.start_ts))}
        </div>
      </div>
    </UpdateDetails>
  );
};
