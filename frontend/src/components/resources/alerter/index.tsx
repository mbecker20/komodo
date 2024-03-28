import { NewResource } from "@components/layouts";
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
import { Config } from "@components/config";
import { DataTable } from "@ui/data-table";
import { ResourceComponents } from "..";
import { Link } from "react-router-dom";
import { fmt_date_with_minutes } from "@lib/utils";
import { Card, CardDescription, CardHeader, CardTitle } from "@ui/card";

const useAlerter = (id?: string) =>
  useRead("ListAlerters", {}).data?.find((d) => d.id === id);

const NewAlerter = () => {
  const { mutateAsync } = useWrite("CreateAlerter");
  const [name, setName] = useState("");
  const [type, setType] = useState<Types.AlerterConfig["type"]>();

  return (
    <NewResource
      type="Alerter"
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
    </NewResource>
  );
};

const SlackAlerterConfig = ({ id }: { id: string }) => {
  const config = useRead("GetAlerter", { alerter: id }).data?.config;
  const [update, set] = useState<Partial<Types.SlackAlerterConfig>>({});
  const { mutate } = useWrite("UpdateAlerter");
  if (!config) return null;

  return (
    <Config
      config={config.params}
      update={update}
      set={set}
      onSave={() => mutate({ id, config: { type: "Slack", params: update } })}
      components={{
        general: {
          general: {
            url: true,
          },
        },
      }}
    />
  );
};

const CustomAlerterConfig = ({ id }: { id: string }) => {
  const config = useRead("GetAlerter", { alerter: id }).data?.config;
  const [update, set] = useState<Partial<Types.CustomAlerterConfig>>({});
  const { mutate } = useWrite("UpdateAlerter");
  if (!config) return null;

  return (
    <Config
      config={config.params}
      update={update}
      set={set}
      onSave={() => mutate({ id, config: { type: "Custom", params: update } })}
      components={{
        general: {
          general: {
            url: true,
          },
        },
      }}
    />
  );
};

const AlerterTable = () => {
  const alerters = useRead("ListAlerters", {}).data;
  return (
    <DataTable
      data={alerters ?? []}
      columns={[
        {
          accessorKey: "id",
          header: "Name",
          cell: ({ row }) => {
            const id = row.original.id;
            return (
              <Link to={`/alerters/${id}`} className="flex items-center gap-2">
                <ResourceComponents.Alerter.Icon id={id} />
                <ResourceComponents.Alerter.Name id={id} />
              </Link>
            );
          },
        },
        { header: "Tags", accessorFn: ({ tags }) => tags.join(", ") },
        {
          header: "Created",
          accessorFn: ({ created_at }) =>
            fmt_date_with_minutes(new Date(created_at)),
        },
      ]}
    />
  );
};

export const AlerterDashboard = () => {
  const alerters_count = useRead("ListAlerters", {}).data?.length;
  return (
    <Link to="/alerters/" className="w-full">
      <Card>
        <CardHeader className="justify-between">
          <div>
            <CardTitle>Alerters</CardTitle>
            <CardDescription>{alerters_count} Total</CardDescription>
          </div>
          <AlarmClock className="w-4 h-4" />
        </CardHeader>
      </Card>
    </Link>
  );
};

export const AlerterComponents: RequiredResourceComponents = {
  Name: ({ id }: { id: string }) => <>{useAlerter(id)?.name}</>,
  Icon: () => <AlarmClock className="w-4 h-4" />,
  Description: ({ id }) => <>{useAlerter(id)?.info.alerter_type} alerter</>,
  Info: ({ id }) => <>{id}</>,
  Page: {
    Config: ({ id }: { id: string }) => {
      const config = useRead("GetAlerter", { alerter: id }).data?.config;
      if (config?.type === "Slack") return <SlackAlerterConfig id={id} />;
      if (config?.type === "Custom") return <CustomAlerterConfig id={id} />;
    },
  },
  Actions: () => null,
  Table: AlerterTable,
  New: () => <NewAlerter />,
  Dashboard: AlerterDashboard,
};
