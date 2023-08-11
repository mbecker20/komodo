import { ResourceCard } from "@layouts/card";
import { Bot, Cloud, Factory, History, Settings } from "lucide-react";
import { ResourceUpdates } from "@components/updates/resource";
import { useAddRecentlyViewed, useRead, useWrite } from "@hooks";
import { Resource } from "@layouts/resource";
import { Link, useParams } from "react-router-dom";
import { Types } from "@monitor/client";
import { useState } from "react";
import { Section } from "@layouts/page";
import { Button } from "@ui/button";
import { ConfirmUpdate } from "@components/config/confirm-update";
import { ConfigAgain, VariantConfig } from "@components/config/again";
import { Input } from "@ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";

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
      {/* <SelectItem value={"Server"}>Server</SelectItem> */}
    </SelectContent>
  </Select>
);

const ConfigInput = ({
  label,
  value,
  onChange,
}: {
  label: string;
  value: string | number;
  onChange: (value: string) => void;
}) => (
  <div className="flex justify-between items-center border-b pb-4">
    <div className="capitalize"> {label} </div>
    <Input
      className="max-w-[400px]"
      type={typeof value === "number" ? "number" : undefined}
      value={value}
      onChange={(e) => onChange(e.target.value)}
      // disabled={loading}
    />
  </div>
);

const BuilderConfig = ({ id }: { id: string }) => {
  const builder = useRead("GetBuilder", { id }).data;
  if (!builder?.config) return null;

  const [type, setT] = useState(builder.config.type);

  const [update, set] = useState<{
    type: Types.BuilderConfig["type"];
    params: Partial<Types.BuilderConfig["params"]>;
  }>({ type: builder.config.type, params: {} });

  const { mutate } = useWrite("UpdateBuilder");

  return (
    <Section
      title="Config"
      icon={<Settings className="w-4 h-4" />}
      actions={
        <div className="flex gap-4">
          <Button
            variant="outline"
            intent="warning"
            onClick={() => set({ type: builder.config.type, params: {} })}
          >
            <History className="w-4 h-4" />
          </Button>
          <ConfirmUpdate
            content={JSON.stringify(update, null, 2)}
            onConfirm={() => {
              mutate({
                id,
                config: update,
              });
            }}
          />
        </div>
      }
    >
      <BuilderTypeSelector
        selected={type ?? builder.config.type}
        onSelect={(type) => setT(type)}
      />
      {type === "Server" && (
        <ConfigAgain
          config={builder.config.params}
          update={update ?? {}}
          components={{
            id: (selected) => <div></div>,
          }}
        />
      )}
      {type === "Aws" && (
        <VariantConfig
          config={builder.config.params}
          update={update ?? {}}
          components={{
            region: (region) => (
              <ConfigInput
                label="Region"
                value={region}
                onChange={(region) => set((u) => ({ ...u, region }))}
              />
            ),
            instance_type: (instance_type) => (
              <ConfigInput
                label="Instance Type"
                value={instance_type}
                onChange={(t) => set((u) => ({ ...u, instance_type: t }))}
              />
            ),

            volume_gb: (volume_gb) => (
              <ConfigInput
                label="Region"
                value={volume_gb}
                onChange={(ami_id) => set((u) => ({ ...u, ami_id }))}
              />
            ),
            ami_id: (ami_id) => (
              <ConfigInput
                label="AMI Id"
                value={ami_id}
                onChange={(ami_id) => set((u) => ({ ...u, ami_id }))}
              />
            ),
            subnet_id: (subnet_id) => (
              <ConfigInput
                label="Subnet Id"
                value={subnet_id}
                onChange={(subnet_id) => set((u) => ({ ...u, subnet_id }))}
              />
            ),
            security_group_ids: (ids) => <div>sec group ids</div>,
            key_pair_name: (key_pair_name) => (
              <ConfigInput
                label="Subnet Id"
                value={key_pair_name}
                onChange={(n) => set((u) => ({ ...u, key_pair_name: n }))}
              />
            ),
            assign_public_ip: () => <div>assign_public_ip</div>,
            // github_accounts: () => <div>github_accounts</div>,
            // docker_accounts: () => <div>docker_accounts</div>,
          }}
        />
      )}
    </Section>
  );
};

export const BuilderPage = () => {
  const id = useParams().builderId;

  if (!id) return null;
  useAddRecentlyViewed("Builder", id);

  return (
    <Resource title={<BuilderName id={id} />} info={<></>} actions={<></>}>
      <ResourceUpdates type="Builder" id={id} />
      <BuilderConfig id={id} />
    </Resource>
  );
};
