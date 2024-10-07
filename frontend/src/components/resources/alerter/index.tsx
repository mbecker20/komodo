import { useRead, useUser } from "@lib/hooks";
import { RequiredResourceComponents } from "@types";
import { AlarmClock } from "lucide-react";
import { Link } from "react-router-dom";
import { Card, CardDescription, CardHeader, CardTitle } from "@ui/card";
import { AlerterConfig } from "./config";
import { DeleteResource, NewResource } from "../common";
import { AlerterTable } from "./table";
import { Types } from "@komodo/client";
import { ResourcePageHeader } from "@components/util";

const useAlerter = (id?: string) =>
  useRead("ListAlerters", {}).data?.find((d) => d.id === id);

export const AlerterComponents: RequiredResourceComponents = {
  list_item: (id) => useAlerter(id),
  resource_links: () => undefined,

  Description: () => <>Route alerts to various endpoints.</>,

  Dashboard: () => {
    const alerters_count = useRead("ListAlerters", {}).data?.length;
    return (
      <Link to="/alerters/" className="w-full">
        <Card className="hover:bg-accent/50 transition-colors cursor-pointer">
          <CardHeader>
            <div className="flex justify-between">
              <div>
                <CardTitle>Alerters</CardTitle>
                <CardDescription>{alerters_count} Total</CardDescription>
              </div>
              <AlarmClock className="w-4 h-4" />
            </div>
          </CardHeader>
        </Card>
      </Link>
    );
  },

  New: () => {
    const is_admin = useUser().data?.admin;
    return is_admin && <NewResource type="Alerter" />;
  },

  Table: ({ resources }) => (
    <AlerterTable alerters={resources as Types.AlerterListItem[]} />
  ),

  Icon: () => <AlarmClock className="w-4 h-4" />,
  BigIcon: () => <AlarmClock className="w-8 h-8" />,

  State: () => null,
  Status: {},

  Info: {
    Type: ({ id }) => {
      const alerter = useAlerter(id);
      return (
        <div className="capitalize">Type: {alerter?.info.endpoint_type}</div>
      );
    },
  },

  Actions: {},

  Page: {},

  Config: AlerterConfig,

  DangerZone: ({ id }) => <DeleteResource type="Alerter" id={id} />,

  ResourcePageHeader: ({ id }) => {
    const alerter = useAlerter(id);
    return (
      <ResourcePageHeader
        intent="None"
        icon={<AlarmClock className="w-8" />}
        name={alerter?.name}
        state={alerter?.info.enabled ? "Enabled" : "Disabled"}
        status={alerter?.info.endpoint_type}
      />
    );
  },
};
