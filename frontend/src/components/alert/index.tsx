import { Section } from "@components/layouts";
import {
  alert_level_intention,
  bg_color_class_by_intention,
} from "@lib/color";
import { useRead, atomWithStorage } from "@lib/hooks";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { useAtom } from "jotai";
import { AlertTriangle } from "lucide-react";
import { AlertsTable } from "./table";
import { Card, CardHeader } from "@ui/card";
import { cn } from "@lib/utils";

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
      {open && <AlertsTable alerts={alerts ?? []} />}
    </Section>
  );
};

export const AlertLevel = ({ level }: { level: Types.SeverityLevel }) => {
  const color = bg_color_class_by_intention(alert_level_intention(level));
  return (
    <Card className={cn("w-fit", color)}>
      <CardHeader className="py-0 px-2">{level}</CardHeader>
    </Card>
  );
};
