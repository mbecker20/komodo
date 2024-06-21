import { AlertsTable } from "@components/alert/table";
import { Page } from "@components/layouts";
import { useRead, useResourceParamType } from "@lib/hooks";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { Label } from "@ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";
import { Switch } from "@ui/switch";
import { AlertTriangle } from "lucide-react";
import { useState } from "react";
import { useParams } from "react-router";

const ALERT_TYPES: { [key: string]: Types.AlertData["type"][] } = {
  Server: ["ServerUnreachable", "ServerCpu", "ServerMem", "ServerDisk"],
  Deployment: ["ContainerStateChange"],
  Build: ["BuildFailed"],
};

const FALLBACK_ALERT_TYPES = [
  ...ALERT_TYPES.Server,
  ...ALERT_TYPES.Deployment,
  ...ALERT_TYPES.Build,
  "AwsBuilderTerminationFailed",
];

export const Alerts = () => {
  const type = useResourceParamType();
  const id = useParams().id as string | undefined;
  const alert_types: string[] = type
    ? ALERT_TYPES[type] ?? FALLBACK_ALERT_TYPES
    : FALLBACK_ALERT_TYPES;

  const [page, setPage] = useState(0);
  const [variant, setVariant] = useState<Types.AlertData["type"] | "All">(
    "All"
  );
  const [onlyOpen, setOnlyOpen] = useState(false);
  const alerts = useRead("ListAlerts", {
    query: {
      "target.type": type,
      "target.id": id,
      "data.type": variant === "All" ? undefined : variant,
      resolved: onlyOpen ? false : undefined,
    },
    page,
  }).data;
  return (
    <Page
      title="Alerts"
      icon={<AlertTriangle className="w-8 h-8" />}
      actions={
        <div className="flex gap-4 items-center justify-end">
          <div
            className="flex gap-3 items-center cursor-pointer"
            onClick={() => setOnlyOpen(!onlyOpen)}
          >
            <Label htmlFor="only-open" className="text-nowrap cursor-pointer">
              Only Open
            </Label>
            <Switch id="only-open" checked={onlyOpen} />
          </div>
          <Select
            value={variant}
            onValueChange={(variant) => {
              setVariant(variant as Types.AlertData["type"] | "All");
            }}
          >
            <SelectTrigger className="w-[200px] overflow-ellipsis">
              <SelectValue placeholder="Alert Type" />
            </SelectTrigger>
            <SelectContent align="end">
              {["All", ...alert_types].map((variant) => (
                <SelectItem value={variant}>{variant}</SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
      }
    >
      <div className="flex flex-col gap-4">
        <AlertsTable alerts={alerts?.alerts ?? []} showResolved />
        <div className="flex gap-4 justify-center items-center text-muted-foreground">
          <Button
            variant="outline"
            onClick={() => setPage(page - 1)}
            disabled={page === 0}
          >
            Prev Page
          </Button>
          Page: {page + 1}
          <Button
            variant="outline"
            onClick={() => alerts?.next_page && setPage(alerts.next_page)}
            disabled={!alerts?.next_page}
          >
            Next Page
          </Button>
        </div>
      </div>
    </Page>
  );
};
