import { AlertLevel } from "@components/alert";
import { UpdateDetails } from "@components/updates/details";
import { Types } from "komodo_client";
import { ColorIntention, text_color_class_by_intention } from "@lib/color";
import { fmt_operation, fmt_version, fmt_date } from "@lib/formatting";
import { useRead } from "@lib/hooks";
import { getUpdateQuery, cn, version_is_none } from "@lib/utils";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@ui/tabs";
import {
  ExternalLink,
  Check,
  X,
  Loader2,
  Milestone,
  Calendar,
  Clock,
} from "lucide-react";
import { Link } from "react-router-dom";

export const ResourceNotifications = ({ type, id }: Types.ResourceTarget) => {
  const deployments = useRead("ListDeployments", {}).data;

  const updates = useRead("ListUpdates", {
    query: getUpdateQuery({ type, id }, deployments),
  }).data;

  const alerts = useRead("ListAlerts", {
    query: getUpdateQuery({ type, id }, deployments),
  }).data;
  const openAlerts = alerts?.alerts.filter((alert) => !alert.resolved);

  const showAlerts = type === "Server";

  return (
    <div className="shrink-0 p-6 pt-5 pr-3 border rounded-md w-full xl:max-w-[500px]">
      <Tabs
        defaultValue={showAlerts && openAlerts?.length ? "alerts" : "updates"}
      >
        <TabsList>
          <TabsTrigger value="updates">Updates</TabsTrigger>
          {showAlerts && <TabsTrigger value="alerts">Alerts</TabsTrigger>}
        </TabsList>
        <div className="mt-2 pr-3 h-[180px] overflow-y-scroll">
          <TabsContent value="updates">
            {updates?.updates.slice(0, 10).map((update) => (
              <Update key={update.id} update={update} />
            ))}
            <ShowAll to={`/updates?type=${type}&id=${id}`} />
          </TabsContent>
          <TabsContent value="alerts">
            {openAlerts && openAlerts.length ? (
              openAlerts
                .slice(0, 10)
                .map((alert) => <Alert alert={alert} key={alert._id?.$oid} />)
            ) : (
              <p className="pl-2 text-sm text-muted-foreground">
                No open alerts
              </p>
            )}
            <ShowAll to={`/updates?type=${type}&id=${id}`} />
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

  const Icon = () =>
    update.status === Types.UpdateStatus.Complete ? (
      update.success ? (
        <Check className="w-4" />
      ) : (
        <X className="w-4" />
      )
    ) : (
      <Loader2 className="w-4 animate-spin" />
    );

  return (
    <UpdateDetails id={update.id}>
      <div className="p-2 flex items-center justify-between gap-4 odd:bg-accent/25 hover:bg-accent cursor-pointer">
        <div
          className={cn(
            "w-full flex items-center gap-2 text-sm font-bold",
            color
          )}
        >
          <Icon />
          {fmt_operation(update.operation)}
        </div>

        <div className="flex items-center gap-8 shrink-0">
          {!version_is_none(update.version) && (
            <div className="flex items-center gap-2 w-full text-xs text-muted-foreground">
              <Milestone className="w-4" />
              {fmt_version(update.version)}
            </div>
          )}
          <div className="flex items-center gap-2 text-xs text-muted-foreground text-right shrink-0">
            <Calendar className="w-4" />
            {fmt_date(new Date(update.start_ts))}
          </div>
        </div>
      </div>
    </UpdateDetails>
  );
};

const Alert = ({ alert }: { alert: Types.Alert }) => (
  <div className="p-2 flex items-center justify-between gap-4 odd:bg-accent/25 hover:bg-accent cursor-pointer">
    <AlertLevel level={alert.level} />
    <div className="w-full font-bold max-w-[40%] overflow-hidden overflow-ellipsis">
      {alert.data.type}
    </div>
    <div className="w-fit flex items-center gap-2 text-xs text-muted-foreground text-right shrink-0">
      <Clock className="w-4" />
      {new Date(alert.ts).toLocaleString()}
    </div>
  </div>
);
