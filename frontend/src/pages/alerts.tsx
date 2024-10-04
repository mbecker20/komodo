import { AlertsTable } from "@components/alert/table";
import { Page } from "@components/layouts";
import { useRead, useResourceParamType } from "@lib/hooks";
import { Types } from "@komodo/client";
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
import {
  AlertTriangle,
  Box,
  ChevronLeft,
  ChevronRight,
  MinusCircle,
} from "lucide-react";
import { useMemo, useState } from "react";
import { useParams } from "react-router";
import { useSearchParams } from "react-router-dom";
import { UsableResource } from "@types";
import { SelectSeparator } from "@radix-ui/react-select";
import { RESOURCE_TARGETS } from "@lib/utils";
import { ResourceComponents } from "@components/resources";
import { ResourceLink } from "@components/resources/common";

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

export const Alerts2 = () => {
  const [page, setPage] = useState(0);
  const [params, setParams] = useSearchParams();

  const { type, id, alert, open } = useMemo(
    () => ({
      type: (params.get("type") as UsableResource) ?? undefined,
      id: params.get("id") ?? undefined,
      alert: (params.get("alert") as Types.AlertData["type"]) ?? undefined,
      open: params.get("open") === "true" || undefined,
    }),
    [params]
  );

  const { data: alerts } = useRead("ListAlerts", {
    query: {
      "target.type": type,
      "target.id": id,
      "data.type": alert,
      resolved: !open,
    },
    page,
  });

  const resources = useRead(`List${type}s`, {}, { enabled: !!type }).data;

  const SelectedResourceIcon = () => {
    if (!type) return null;
    const Icon = ResourceComponents[type].Icon;
    return <Icon />;
  };

  const alert_types: string[] = type
    ? ALERT_TYPES[type] ?? FALLBACK_ALERT_TYPES
    : FALLBACK_ALERT_TYPES;

  return (
    <Page
      title="Alerts"
      icon={<AlertTriangle className="w-8" />}
      actions={
        <>
          <div className="flex items-center md:justify-end gap-4 flex-wrap">
            {/* resource type */}
            <Select
              value={type ?? "all"}
              onValueChange={(type) => {
                const p = new URLSearchParams(params.toString());
                type === "all" ? p.delete("type") : p.set("type", type);
                p.delete("id");
                p.delete("operation");
                setParams(p);
              }}
            >
              <SelectTrigger className="w-48">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">
                  <div className="flex items-center gap-2">
                    <Box className="w-4 text-muted-foreground" />
                    All Resources
                  </div>
                </SelectItem>
                <SelectSeparator />
                {RESOURCE_TARGETS.map((type) => {
                  const Icon = ResourceComponents[type].Icon;
                  return (
                    <SelectItem key={type} value={type}>
                      <div className="flex items-center gap-2">
                        <span className="text-muted-foreground">
                          <Icon />
                        </span>
                        {type}
                      </div>
                    </SelectItem>
                  );
                })}
              </SelectContent>
            </Select>

            {/* resource id */}
            <Select
              value={id ? id : type ? "all" : undefined}
              disabled={!type}
              onValueChange={(id) => {
                const p = new URLSearchParams(params.toString());
                id === "all" ? p.delete("id") : p.set("id", id);
                setParams(p);
              }}
            >
              <SelectTrigger className="w-64">
                <SelectValue placeholder="Resources" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem value="all">
                  <div className="flex items-center gap-2">
                    <span className="text-muted-foreground">
                      <SelectedResourceIcon />
                    </span>
                    All {type}s
                  </div>
                </SelectItem>
                <SelectSeparator />
                {resources?.map((resource) => (
                  <SelectItem key={resource.id} value={resource.id}>
                    <ResourceLink type={type} id={resource.id} />
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>

            {/* operation */}
            <Select
              value={alert}
              onValueChange={(alert) => {
                const p = new URLSearchParams(params.toString());
                alert === "All" ? p.delete("alert") : p.set("alert", alert);
                setParams(p);
              }}
            >
              <SelectTrigger className="w-64 overflow-ellipsis">
                <SelectValue placeholder="Alert Type" />
              </SelectTrigger>
              <SelectContent align="end">
                {["All", ...alert_types].map((variant) => (
                  <SelectItem key={variant} value={variant}>
                    {variant}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>

            {/* only open */}
            <div
              className="px-4 h-9 flex items-center gap-4 border rounded-md"
              onClick={() => {
                const p = new URLSearchParams(params.toString());
                open ? p.delete("open") : p.set("open", "true");
                setParams(p);
              }}
            >
              <p className="text-sm text-muted-foreground">Only Open</p>
              <Switch checked={open} className="pointer-events-none" />
            </div>

            {/* reset */}
            <Button
              size="icon"
              onClick={() => setParams({})}
              variant="secondary"
            >
              <MinusCircle className="w-4" />
            </Button>
          </div>
        </>
      }
    >
      <div className="flex flex-col gap-2">
        <AlertsTable alerts={alerts?.alerts ?? []} showResolved />
        <div className="flex gap-4 items-center">
          <Button
            variant="outline"
            onClick={() => setPage(page - 1)}
            disabled={page === 0}
            size="icon"
          >
            <ChevronLeft className="w-4" />
          </Button>
          {Array.from(new Array(page + 1)).map((_, i) => (
            <Button
              key={i}
              onClick={() => setPage(i)}
              variant={page === i ? "secondary" : "outline"}
            >
              {i + 1}
            </Button>
          ))}
          {/* Page: {page + 1} */}
          <Button
            variant="outline"
            onClick={() => alerts?.next_page && setPage(alerts.next_page)}
            disabled={!alerts?.next_page}
            size="icon"
          >
            <ChevronRight className="w-4" />
          </Button>
        </div>
      </div>
    </Page>
  );
};
