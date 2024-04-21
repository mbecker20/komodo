import { NewLayout } from "@components/layouts";
import { useRead, useTagsFilter, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { RequiredResourceComponents } from "@types";
import { Card, CardDescription, CardHeader, CardTitle } from "@ui/card";
import { DataTable, SortableHeader } from "@ui/data-table";
import { Input } from "@ui/input";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";
import { Cloud, Bot, Factory } from "lucide-react";
import { useState } from "react";
import { Link } from "react-router-dom";
import { BuilderConfig } from "./config";
import { DeleteResource, ResourceLink } from "../common";

const useBuilder = (id?: string) =>
  useRead("ListBuilders", {}).data?.find((d) => d.id === id);

export const BuilderComponents: RequiredResourceComponents = {
  Dashboard: () => {
    const builders_count = useRead("ListBuilders", {}).data?.length;
    return (
      <Link to="/builders/" className="w-full">
        <Card className="hover:bg-accent/50 transition-colors cursor-pointer">
          <CardHeader>
            <div className="flex justify-between">
              <div>
                <CardTitle>Builders</CardTitle>
                <CardDescription>{builders_count} Total</CardDescription>
              </div>
              <Factory className="w-4 h-4" />
            </div>
          </CardHeader>
        </Card>
      </Link>
    );
  },

  New: () => {
    const { mutateAsync } = useWrite("CreateBuilder");
    const [name, setName] = useState("");
    const [type, setType] = useState<Types.BuilderConfig["type"]>();

    return (
      <NewLayout
        entityType="Builder"
        onSuccess={async () =>
          !!type && mutateAsync({ name, config: { type, params: {} } })
        }
        enabled={!!name && !!type}
      >
        <div className="grid md:grid-cols-2">
          Name
          <Input
            placeholder="builder-name"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
        </div>
        <div className="grid md:grid-cols-2">
          Builder Type
          <Select
            value={type}
            onValueChange={(value) => setType(value as typeof type)}
          >
            <SelectTrigger>
              <SelectValue placeholder="Select Type" />
            </SelectTrigger>
            <SelectContent>
              <SelectGroup>
                <SelectItem value="Aws">Aws</SelectItem>
                <SelectItem value="Server">Server</SelectItem>
              </SelectGroup>
            </SelectContent>
          </Select>
        </div>
      </NewLayout>
    );
  },

  Table: ({ search }) => {
    const tags = useTagsFilter();
    const builders = useRead("ListBuilders", {}).data;
    const searchSplit = search?.split(" ") || [];
    return (
      <DataTable
        tableKey="builders"
        data={
          builders?.filter(
            (resource) =>
              tags.every((tag) => resource.tags.includes(tag)) &&
              (searchSplit.length > 0
                ? searchSplit.every((search) => resource.name.includes(search))
                : true)
          ) ?? []
        }
        columns={[
          {
            accessorKey: "name",
            header: ({ column }) => (
              <SortableHeader column={column} title="Name" />
            ),
            cell: ({ row }) => (
              <ResourceLink type="Builder" id={row.original.id} />
            ),
          },
          {
            header: "Provider",
            accessorKey: "info.provider",
          },
          {
            header: "Instance Type",
            accessorKey: "info.instance_type",
          },
          { header: "Tags", accessorFn: ({ tags }) => tags.join(", ") },
        ]}
      />
    );
  },

  Name: ({ id }: { id: string }) => <>{useBuilder(id)?.name}</>,
  name: (id) => useBuilder(id)?.name,

  Icon: () => <Factory className="w-4 h-4" />,

  Status: {},

  Info: {
    Provider: ({ id }) => (
      <div className="flex items-center gap-2">
        <Cloud className="w-4 h-4" />
        {useBuilder(id)?.info.provider}
      </div>
    ),
    InstanceType: ({ id }) => (
      <div className="flex items-center gap-2">
        <Bot className="w-4 h-4" />
        {useBuilder(id)?.info.instance_type ?? "N/A"}
      </div>
    ),
  },

  Actions: {},

  Page: {},

  Config: BuilderConfig,

  DangerZone: ({ id }) => <DeleteResource type="Builder" id={id} />,
};
