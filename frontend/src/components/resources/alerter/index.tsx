import { NewLayout } from "@components/layouts";
import { useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectGroup,
  SelectItem,
} from "@ui/select";
import { RequiredResourceComponents } from "@types";
import { Input } from "@ui/input";
import { AlarmClock } from "lucide-react";
import { useState } from "react";
import { Link } from "react-router-dom";
import { Card, CardDescription, CardHeader, CardTitle } from "@ui/card";
import { AlerterConfig } from "./config";
import { DeleteResource } from "../common";
import { AlerterTable } from "./table";

const useAlerter = (id?: string) =>
  useRead("ListAlerters", {}).data?.find(
    (d) => d.id === id
  );

export const AlerterComponents: RequiredResourceComponents = {
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
    const { mutateAsync } = useWrite("CreateAlerter");
    const [name, setName] = useState("");
    const [type, setType] = useState<Types.AlerterConfig["type"]>();

    return (
      <NewLayout
        entityType="Alerter"
        onSuccess={async () =>
          !!type && mutateAsync({ name, config: { type, params: {} } })
        }
        enabled={!!name && !!type}
      >
        <div className="grid md:grid-cols-2">
          Name
          <Input
            placeholder="alerter-name"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
        </div>
        <div className="grid md:grid-cols-2">
          Alerter Type
          <Select
            value={type}
            onValueChange={(value) => setType(value as typeof type)}
          >
            <SelectTrigger>
              <SelectValue placeholder="Select Type" />
            </SelectTrigger>
            <SelectContent>
              <SelectGroup>
                <SelectItem value="Slack">Slack</SelectItem>
                <SelectItem value="Custom">Custom</SelectItem>
              </SelectGroup>
            </SelectContent>
          </Select>
        </div>
      </NewLayout>
    );
  },

  Table: AlerterTable,

  Name: ({ id }: { id: string }) => <>{useAlerter(id)?.name}</>,
  name: (id) => useAlerter(id)?.name,

  Icon: () => <AlarmClock className="w-4 h-4" />,

  Status: {},

  Info: {
    Type: ({ id }) => {
      const alerter = useAlerter(id);
      return (
        <div className="capitalize">Type: {alerter?.info.alerter_type}</div>
      );
    },
  },

  Actions: {},

  Page: {},

  Config: AlerterConfig,

  DangerZone: ({ id }) => <DeleteResource type="Alerter" id={id} />,
};
