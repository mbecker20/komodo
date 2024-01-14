import { ConfigInner } from "@components/config";
import { InputList, ResourceSelector } from "@components/config/util";
import { NewResource } from "@components/layouts";
import { useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { Icon } from "@radix-ui/react-select";
import { RequiredResourceComponents } from "@types";
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
import { Cloud, Bot, Factory, Link } from "lucide-react";
import { useState } from "react";

const useBuilder = (id?: string) =>
  useRead("ListBuilders", {}).data?.find((d) => d.id === id);

const AwsBuilderConfig = ({ id }: { id: string }) => {
  const config = useRead("GetBuilder", { id }).data?.config;
  const [update, set] = useState<Partial<Types.AwsBuilderConfig>>({});
  const { mutate } = useWrite("UpdateBuilder");
  if (!config) return null;

  return (
    <ConfigInner
      config={config.params as Types.AwsBuilderConfig}
      update={update}
      set={set}
      onSave={() => mutate({ id, config: { type: "Aws", params: update } })}
      components={{
        general: {
          general: {
            region: true,
            instance_type: true,
            volume_gb: true,
            ami_id: true,
            subnet_id: true,
            key_pair_name: true,
            assign_public_ip: true,
            security_group_ids: (values, set) => (
              <InputList field="security_group_ids" values={values} set={set} />
            ),
            docker_accounts: (accounts, set) => (
              <InputList
                field="docker_accounts"
                values={accounts ?? []}
                set={set}
              />
            ),
            github_accounts: (accounts, set) => (
              <InputList
                field="github_accounts"
                values={accounts ?? []}
                set={set}
              />
            ),
          },
        },
      }}
    />
  );
};

const ServerBuilderConfig = ({ id }: { id: string }) => {
  const config = useRead("GetBuilder", { id }).data?.config;
  const [update, set] = useState<Partial<Types.ServerBuilderConfig>>({});
  const { mutate } = useWrite("UpdateBuilder");
  if (!config) return null;

  return (
    <ConfigInner
      config={config.params as Types.ServerBuilderConfig}
      update={update}
      set={set}
      onSave={() => mutate({ id, config: { type: "Server", params: update } })}
      components={{
        general: {
          general: {
            id: (id, set) => (
              <div className="flex justify-between items-center border-b pb-4">
                Select Server
                <ResourceSelector
                  type="Server"
                  selected={id}
                  onSelect={(id) => set({ id })}
                />
              </div>
            ),
          },
        },
      }}
    />
  );
};

const NewBuilder = () => {
  const { mutateAsync } = useWrite("CreateBuilder");
  const [name, setName] = useState("");
  const [type, setType] = useState<Types.BuilderConfig["type"]>();

  return (
    <NewResource
      type="Builder"
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
};
const Name = ({ id }: { id: string }) => <>{useBuilder(id)?.name}</>;

export const Builder: RequiredResourceComponents = {
  Name,
  Description: ({ id }) => <>{id}</>,
  Info: ({ id }) => (
    <>
      <div className="flex items-center gap-2">
        <Cloud className="w-4 h-4" />
        {useBuilder(id)?.info.provider}
      </div>
      <div className="flex items-center gap-2">
        <Bot className="w-4 h-4" />
        {useBuilder(id)?.info.instance_type ?? "N/A"}
      </div>
    </>
  ),
  Icon: () => <Factory className="w-4 h-4" />,
  Page: {
    Config: ({ id }) => {
      const config = useRead("GetBuilder", { id }).data?.config;
      if (config?.type === "Aws") return <AwsBuilderConfig id={id} />;
      if (config?.type === "Server") return <ServerBuilderConfig id={id} />;
    },
  },
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
                  to={`/builders/${id}`}
                  className="flex items-center gap-2"
                >
                  <Icon id={id} />
                  <Name id={id} />
                </Link>
              );
            },
          },
          { header: "Tags", accessorFn: ({ tags }) => tags.join(", ") },
        ]}
      />
    );
  },
  Actions: () => null,
  New: () => <NewBuilder />,
};
