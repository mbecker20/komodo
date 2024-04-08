import { NewLayout, Section } from "@components/layouts";
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
import { AlarmClock, AlertTriangle } from "lucide-react";
import { useState } from "react";
import { DataTable } from "@ui/data-table";
import { Link } from "react-router-dom";
import { Card, CardDescription, CardHeader, CardTitle } from "@ui/card";
import { AlerterConfig } from "./config";
import { TagsWithBadge, useTagsFilter } from "@components/tags";
import { CopyResource, DeleteResource, ResourceLink } from "../common";

const useAlerter = (id?: string) =>
  useRead("ListAlerters", {}).data?.find((d) => d.id === id);

export const AlerterComponents: RequiredResourceComponents = {
  Name: ({ id }: { id: string }) => <>{useAlerter(id)?.name}</>,
  Icon: () => <AlarmClock className="w-4 h-4" />,
  Link: ({ id }) => <ResourceLink type="Alerter" id={id} />,
  Info: [],
  Status: () => <></>,
  Page: {
    Config: AlerterConfig,
    Danger: ({ id }) => (
      <Section
        title="Danger Zone"
        icon={<AlertTriangle className="w-4 h-4" />}
        actions={<CopyResource type="Alerter" id={id} />}
      >
        <DeleteResource type="Alerter" id={id} />
      </Section>
    ),
  },
  Actions: [],
  Table: ({ search }) => {
    const tags = useTagsFilter();
    const alerters = useRead("ListAlerters", {}).data;
    const searchSplit = search?.split(" ") || [];
    return (
      <DataTable
        data={
          alerters?.filter((resource) =>
            tags.every((tag) => resource.tags.includes(tag)) &&
            searchSplit.length > 0
              ? searchSplit.every((search) => resource.name.includes(search))
              : true
          ) ?? []
        }
        columns={[
          {
            header: "Name",
            cell: ({ row }) => <AlerterComponents.Link id={row.original.id} />,
          },
          {
            header: "Type",
            accessorKey: "info.alerter_type",
          },
          {
            header: "Tags",
            cell: ({ row }) => {
              return (
                <div className="flex gap-1">
                  <TagsWithBadge tag_ids={row.original.tags} />
                </div>
              );
            },
          },
        ]}
      />
    );
  },
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
};
