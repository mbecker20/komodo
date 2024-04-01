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
import { DataTable } from "@ui/data-table";
import { ResourceComponents } from "..";
import { Link } from "react-router-dom";
import { Card, CardDescription, CardHeader, CardTitle } from "@ui/card";
import { AlerterConfig } from "./config";
import { ResourceLink } from "@components/util";

const useAlerter = (id?: string) =>
  useRead("ListAlerters", {}).data?.find((d) => d.id === id);

export const AlerterComponents: RequiredResourceComponents = {
  Name: ({ id }: { id: string }) => <>{useAlerter(id)?.name}</>,
  Icon: () => <AlarmClock className="w-4 h-4" />,
  Description: ({ id }) => <>{useAlerter(id)?.info.alerter_type} alerter</>,
  Link: ({ id }) => <ResourceLink type="Alerter" id={id} />,
  Info: [],
  Status: () => <></>,
  Page: {
    Config: AlerterConfig,
  },
  Actions: [],
  Table: () => {
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
                <Link
                  to={`/alerters/${id}`}
                  className="flex items-center gap-2"
                >
                  <ResourceComponents.Alerter.Icon id={id} />
                  <ResourceComponents.Alerter.Name id={id} />
                </Link>
              );
            },
          },
          { header: "Tags", accessorFn: ({ tags }) => tags.join(", ") },
        ]}
      />
    );
  },
  Dashboard: () => {
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
  },
  New: () => {
    const { mutateAsync } = useWrite("CreateAlerter");
    const [name, setName] = useState("");
    const [type, setType] = useState<Types.AlerterConfig["type"]>();

    return (
      <NewResource
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
      </NewResource>
    );
  },
};
