import { Section } from "@components/layouts";
import { alert_level_intention } from "@lib/color";
import { useRead, atomWithStorage } from "@lib/hooks";
import { Types } from "komodo_client";
import { Button } from "@ui/button";
import { useAtom } from "jotai";
import { AlertTriangle } from "lucide-react";
import { AlertsTable } from "./table";
import { StatusBadge } from "@components/util";

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
          {open ? "hide" : "show"}
        </Button>
      }
    >
      {open && <AlertsTable alerts={alerts ?? []} />}
    </Section>
  );
};

export const AlertLevel = ({
  level,
}: {
  level: Types.SeverityLevel | undefined;
}) => {
  if (!level) return null;
  return <StatusBadge text={level} intent={alert_level_intention(level)} />;
};
