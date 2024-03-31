import { NewResource } from "@components/layouts";
import { useTagsFilter } from "@components/tags";
import { useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { RequiredResourceComponents } from "@types";
import { Card, CardDescription, CardHeader, CardTitle } from "@ui/card";
import { DataTable } from "@ui/data-table";
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

const useBuilder = (id?: string) =>
  useRead("ListBuilders", {}).data?.find((d) => d.id === id);

export const BuilderComponents: RequiredResourceComponents = {
  Name: ({ id }: { id: string }) => <>{useBuilder(id)?.name}</>,
  Description: () => <></>,
  Info: [
    ({ id }) => (
      <div className="flex items-center gap-2">
        <Cloud className="w-4 h-4" />
        {useBuilder(id)?.info.provider}
      </div>
    ),
    ({ id }) => (
      <div className="flex items-center gap-2">
        <Bot className="w-4 h-4" />
        {useBuilder(id)?.info.instance_type ?? "N/A"}
      </div>
    ),
  ],
  Icon: () => <Factory className="w-4 h-4" />,
  Status: () => <>Builder</>,
  Actions: () => <></>,
  Page: {
    Config: BuilderConfig,
  },
  Table: () => {
    const tags = useTagsFilter();
    const builders = useRead("ListBuilders", {}).data;
    return (
      <DataTable
        data={
          builders?.filter((builder) =>
            tags.every((tag) => builder.tags.includes(tag))
          ) ?? []
        }
        columns={[
          {
            accessorKey: "id",
            header: "Name",
            cell: ({ row }) => {
              const id = row.original.id;
              return (
                <Link
                  to={`/builders/${id}`}
                  className="flex items-center gap-2"
                >
                  <Factory className="w-4 h-4" />
                  <BuilderComponents.Name id={id} />
                </Link>
              );
            },
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
  New: () => {
    const { mutateAsync } = useWrite("CreateBuilder");
    const [name, setName] = useState("");
    const [type, setType] = useState<Types.BuilderConfig["type"]>();

    return (
      <NewResource
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
      </NewResource>
    );
  },
  Dashboard: () => {
    const builders_count = useRead("ListBuilders", {}).data?.length;
    return (
      <Link to="/builders/" className="w-full">
        <Card>
          <CardHeader className="justify-between">
            <div>
              <CardTitle>Builders</CardTitle>
              <CardDescription>{builders_count} Total</CardDescription>
            </div>
            <Factory className="w-4 h-4" />
          </CardHeader>
        </Card>
      </Link>
    );
  },
};
