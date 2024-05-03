import { Section } from "@components/layouts";
import {
  alert_level_intention,
  text_color_class_by_intention,
} from "@lib/color";
import { useRead } from "@lib/hooks";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { useAtom } from "jotai";
import { atomWithStorage } from "jotai/utils";
import { AlertTriangle } from "lucide-react";
import { AlertsTable } from "./table";

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
  return (
    <div
      className={text_color_class_by_intention(alert_level_intention(level))}
    >
      {level}
    </div>
  );
};
