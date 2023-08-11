import { ResourceCard } from "@layouts/card";
import { Bot, Cloud, Factory } from "lucide-react";
import { ResourceUpdates } from "@components/updates/resource";
import { useAddRecentlyViewed, useRead, useWrite } from "@hooks";
import { Resource } from "@layouts/resource";
import { Link, useParams } from "react-router-dom";
import { Types } from "@monitor/client";
import { useState } from "react";
import { ConfigLayout } from "@layouts/page";
import { ConfigAgain } from "@components/config/again";
import { Input } from "@ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";
import { ServersSelector } from "@resources/deployment/config";
import { Card, CardContent, CardHeader, CardTitle } from "@ui/card";
import { Button } from "@ui/button";
import { ConfigInput } from "@components/config/util";

export const BuilderName = ({ id }: { id: string }) => {
  const builders = useRead("ListBuilders", {}).data;
  const builder = builders?.find((b) => b.id === id);
  return <>{builder?.name}</>;
};

export const BuilderCard = ({ id }: { id: string }) => {
  const builders = useRead("ListBuilders", {}).data;
  const builder = builders?.find((b) => b.id === id);
  if (!builder) return null;
  return (
    <Link to={`/builders/${builder.id}`}>
      <ResourceCard
        title={builder.name}
        description={"some description"}
        statusIcon={<Factory className="w-4 h-4" />}
      >
        <div className="flex flex-col text-muted-foreground text-sm">
          <div className="flex items-center gap-2">
            <Cloud className="w-4 h-4" />
            {builder.info.provider}
          </div>
          <div className="flex items-center gap-2">
            <Bot className="w-4 h-4" />
            {builder.info.instance_type ?? "n/a"}
          </div>
        </div>
      </ResourceCard>
    </Link>
  );
};

const BuilderTypeSelector = ({
  selected,
  onSelect,
}: {
  selected: Types.BuilderConfig["type"] | undefined;
  onSelect: (type: Types.BuilderConfig["type"]) => void;
}) => (
  <Select value={selected || undefined} onValueChange={onSelect}>
    <SelectTrigger className="max-w-[150px]">
      <SelectValue placeholder="Select Type" />
    </SelectTrigger>
    <SelectContent>
      <SelectItem value={"Aws"}>Aws</SelectItem>
      <SelectItem value={"Server"}>Server</SelectItem>
    </SelectContent>
  </Select>
);

const ServerConfig = ({
  config,
  update,
  set,
}: {
  config: Types.ServerBuilderConfig;
  update: Partial<Types.ServerBuilderConfig>;
  set: (update: Partial<Types.ServerBuilderConfig>) => void;
}) => (
  <ConfigAgain
    config={config}
    update={update}
    components={{
      id: (id) => (
        <div className="flex justify-between items-center border-b pb-4">
          Select Server
          <ServersSelector selected={id} onSelect={(id) => set({ id })} />
        </div>
      ),
    }}
  />
);

const AwsBuilderConfig = ({
  config,
  update,
  set,
}: {
  config: Types.AwsBuilderConfig;
  update: Partial<Types.AwsBuilderConfig>;
  set: (update: Partial<Types.AwsBuilderConfig>) => void;
}) => (
  <ConfigAgain
    config={config}
    update={update}
    components={{
      region: (region) => (
        <ConfigInput
          label="Region"
          value={region}
          onChange={(region) => set({ region })}
        />
      ),
      instance_type: (instance_type) => (
        <ConfigInput
          label="Instance Type"
          value={instance_type}
          onChange={(instance_type) => set({ instance_type })}
        />
      ),

      volume_gb: (volume_gb) => (
        <ConfigInput
          label="Region"
          value={volume_gb}
          onChange={(ami_id) => set({ ami_id })}
        />
      ),
      ami_id: (ami_id) => (
        <ConfigInput
          label="AMI Id"
          value={ami_id}
          onChange={(ami_id) => set({ ami_id })}
        />
      ),
      subnet_id: (subnet_id) => (
        <ConfigInput
          label="Subnet Id"
          value={subnet_id}
          onChange={(subnet_id) => set({ subnet_id })}
        />
      ),
      key_pair_name: (key_pair_name) => (
        <ConfigInput
          label="Subnet Id"
          value={key_pair_name}
          onChange={(n) => set({ key_pair_name })}
        />
      ),
      assign_public_ip: () => <div>assign_public_ip</div>,
      // security_group_ids: (ids) => <div>sec group ids</div>,
      // github_accounts: () => <div>github_accounts</div>,
      // docker_accounts: () => <div>docker_accounts</div>,
    }}
  />
);

const BuilderConfig = ({
  id,
  config,
}: {
  id: string;
  config: Types.BuilderConfig;
}) => {
  const [update, set] = useState({ type: config.type, params: {} });
  const { mutate } = useWrite("UpdateBuilder");
  return (
    <ConfigLayout
      content={update}
      onConfirm={() => mutate({ id, config: update })}
      onReset={() => set({ type: config.type, params: {} })}
    >
      <div className="flex gap-4">
        <div className="flex flex-col gap-4 w-[300px]">
          <Button>General</Button>
        </div>
        <Card>
          <CardHeader className="border-b">
            <CardTitle>General</CardTitle>
          </CardHeader>
          <CardContent className="flex flex-col gap-4 mt-6">
            <div className="flex justify-between items-center border-b pb-4">
              Builder Type
              <BuilderTypeSelector
                selected={update.type}
                onSelect={(type) => set({ type, params: {} })}
              />
            </div>
            {update.type === "Server" && (
              <ServerConfig
                config={config.params as Types.ServerBuilderConfig}
                update={update.params}
                set={(u) =>
                  set((p) => ({ ...p, params: { ...p.params, ...u } }))
                }
              />
            )}
            {update.type === "Aws" && (
              <AwsBuilderConfig
                config={config.params as Types.AwsBuilderConfig}
                update={update.params}
                set={(u) =>
                  set((p) => ({ ...p, params: { ...p.params, ...u } }))
                }
              />
            )}
          </CardContent>
        </Card>
      </div>
    </ConfigLayout>
  );
};

const BCWrapper = ({ id }: { id: string }) => {
  const config = useRead("GetBuilder", { id }).data?.config;
  if (!config) return null;
  return <BuilderConfig id={id} config={config} />;
};

export const BuilderPage = () => {
  const id = useParams().builderId;

  if (!id) return null;
  useAddRecentlyViewed("Builder", id);

  return (
    <Resource title={<BuilderName id={id} />} info={<></>} actions={<></>}>
      <ResourceUpdates type="Builder" id={id} />
      <BCWrapper id={id} />
    </Resource>
  );
};
