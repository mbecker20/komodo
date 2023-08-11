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
  <div className="flex justify-between items-center border-b pb-4">
    Builder Type
    <Select value={selected || undefined} onValueChange={onSelect}>
      <SelectTrigger className="max-w-[150px]">
        <SelectValue placeholder="Select Type" />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value={"Aws"}>Aws</SelectItem>
        <SelectItem value={"Server"}>Server</SelectItem>
      </SelectContent>
    </Select>
  </div>
);

const BuilderConfig = ({
  id,
  config,
}: {
  id: string;
  config: Types.BuilderConfig;
}) => {
  const [update, setUpdate] = useState({ type: config.type, params: {} });
  const { mutate } = useWrite("UpdateBuilder");
  return (
    <ConfigLayout
      content={update}
      onConfirm={() => mutate({ id, config: update })}
      onReset={() => setUpdate({ type: config.type, params: {} })}
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
            <BuilderTypeSelector
              selected={update.type}
              onSelect={(type) => setUpdate({ type, params: {} })}
            />

            {/* Server Builder */}
            {update.type === "Server" && (
              <ConfigAgain
                config={config.params as Types.ServerBuilderConfig}
                update={update.params}
                set={(u) =>
                  setUpdate((p) => ({ ...p, params: { ...p.params, ...u } }))
                }
                components={{
                  id: (id, set) => (
                    <div className="flex justify-between items-center border-b pb-4">
                      Select Server
                      <ServersSelector
                        selected={id}
                        onSelect={(id) => set({ id })}
                      />
                    </div>
                  ),
                }}
              />
            )}

            {/* Aws Builder */}
            {update.type === "Aws" && (
              <ConfigAgain
                config={config.params as Types.AwsBuilderConfig}
                update={update.params}
                set={(u) =>
                  setUpdate((p) => ({ ...p, params: { ...p.params, ...u } }))
                }
                components={{
                  region: true,
                  instance_type: true,
                  volume_gb: true,
                  ami_id: true,
                  subnet_id: true,
                  key_pair_name: true,
                }}
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
