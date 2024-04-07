import { Section } from "@components/layouts";
import { ResourceComponents } from "@components/resources";
import { ResourceLink } from "@components/util";
import {
  alert_level_intention,
  text_color_class_by_intention,
} from "@lib/color";
import { fmt_date_with_minutes } from "@lib/formatting";
import { useRead } from "@lib/hooks";
import { Types } from "@monitor/client";
import { UsableResource } from "@types";
import { Button } from "@ui/button";
import { DataTable } from "@ui/data-table";
import { Dialog, DialogContent, DialogHeader, DialogTrigger } from "@ui/dialog";
import { useAtom } from "jotai";
import { atomWithStorage } from "jotai/utils";
import { AlertTriangle } from "lucide-react";
import { useState } from "react";

const openAtom = atomWithStorage("show-alerts-v0", true);

export const OpenAlerts = () => {
  const [open, setOpen] = useAtom(openAtom);
  const alerts = useRead("ListAlerts", { query: { resolved: false } }).data
    ?.alerts;
  if (!alerts || alerts.length === 0) return null;
  return (
    <Section
      title="Open Alerts"
      icon={<AlertTriangle className="w-4 h-4" />}
      actions={
        <Button variant="ghost" onClick={() => setOpen(!open)}>
          {open ? "close" : "open"}
        </Button>
      }
    >
      {open && (
        <DataTable
          data={alerts ?? []}
          columns={[
            {
              header: "Details",
              cell: ({ row }) =>
                row.original._id?.$oid && (
                  <AlertDetailsDialog id={row.original._id?.$oid} />
                ),
            },
            {
              header: "Target",
              cell: ({ row }) => {
                switch (row.original.target.type) {
                  case "Server":
                    return (
                      <ResourceComponents.Server.Link
                        id={row.original.target.id}
                      />
                    );
                  default:
                    return "Unknown";
                }
              },
            },
            {
              header: "Level",
              cell: ({ row }) => <AlertLevel level={row.original.level} />,
            },
            {
              header: "Alert",
              accessorKey: "variant",
            },
            {
              header: "Open Since",
              accessorFn: ({ ts }) => fmt_date_with_minutes(new Date(ts)),
            },
          ]}
        />
      )}
    </Section>
  );
};

const AlertLevel = ({ level }: { level: Types.SeverityLevel }) => {
  return (
    <div
      className={text_color_class_by_intention(alert_level_intention(level))}
    >
      {level}
    </div>
  );
};

const AlertDetailsDialog = ({ id }: { id: string }) => {
  const [open, set] = useState(false);
  const alert = useRead("ListAlerts", {}).data?.alerts.find(
    (alert) => alert._id?.$oid === id
  );
  return (
    <Dialog open={open} onOpenChange={set}>
      <DialogTrigger asChild>
        <Button variant="secondary" className="items-center gap-2">
          Details
        </Button>
      </DialogTrigger>
      <DialogContent className="w-fit min-w-[30vw] max-w-[90vw]">
        {alert && (
          <>
            <DialogHeader className="flex-row justify-between w-full">
              {alert && (
                <>
                  <div className="flex gap-4 items-center">
                    <ResourceLink
                      type={alert.target.type as UsableResource}
                      id={alert.target.id}
                    />
                    <AlertLevel level={alert.level} />
                  </div>
                  <div className="text-muted-foreground">
                    {fmt_date_with_minutes(new Date(alert.ts))}
                  </div>
                </>
              )}
            </DialogHeader>
            <pre>{JSON.stringify(alert.data, undefined, 2)}</pre>
          </>
        )}
      </DialogContent>
    </Dialog>
  );
};
