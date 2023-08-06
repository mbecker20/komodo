import { ResourceCard } from "@layouts/card";
import {
  Bot,
  Cloud,
  Factory,
  History,
  PlusCircle,
  Settings,
} from "lucide-react";
import { ResourceUpdates } from "@components/updates/resource";
import { useAddRecentlyViewed, useRead, useWrite } from "@hooks";
import { Resource } from "@layouts/resource";
import { Link, useParams } from "react-router-dom";
import { Types } from "@monitor/client";
import { useState } from "react";
import { Section } from "@layouts/page";
import { Button } from "@ui/button";
import { ConfirmUpdate } from "@components/config/confirm-update";
import { Configuration } from "@components/config";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";
import { Input } from "@ui/input";

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
            {builder.provider}
          </div>
          <div className="flex items-center gap-2">
            <Bot className="w-4 h-4" />
            {builder.instance_type ?? "n/a"}
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

const BuilderConfig = ({ id }: { id: string }) => {
  const builder = useRead("GetBuilder", { id }).data;
  const [update, setUpdate] = useState<Partial<Types.BuilderConfig>>({});
  const { mutate, isLoading } = useWrite("UpdateBuilder");

  if (!builder?.config) return null;

  return (
    <Section
      title="Config"
      icon={<Settings className="w-4 h-4" />}
      actions={
        <div className="flex gap-4">
          <Button
            variant="outline"
            intent="warning"
            onClick={() => setUpdate({})}
          >
            <History className="w-4 h-4" />
          </Button>
          <ConfirmUpdate
            content={JSON.stringify(update, null, 2)}
            onConfirm={() => mutate({ config: update as any, id })}
          />
        </div>
      }
    >
      <Configuration
        config={builder.config}
        loading={isLoading}
        update={update}
        set={(input) => setUpdate((update) => ({ ...update, ...input }))}
        layout={{
          general: ["type", "params"],
        }}
        overrides={{
          type: (type, set) => (
            <BuilderTypeSelector
              selected={type}
              onSelect={(type) => set({ ...builder, type })}
            />
          ),
          params: (_, set) => (
            <Configuration
              config={builder.config.params}
              loading={isLoading}
              update={update?.params ?? {}}
              set={(newparams) =>
                set({ params: { ...update.params, ...newparams } } as any)
              }
              overrides={{
                security_group_ids: (ids, setIds) => (
                  <div className="flex flex-col gap-4 border-b pb-4">
                    <div>Security group ids</div>
                    {ids.map((id, i) => (
                      <Input
                        value={id}
                        onChange={(e) => {
                          ids[i] = e.target.value;
                          setIds({ security_group_ids: [...ids] });
                        }}
                      />
                    ))}
                    <Button variant="outline" intent="success">
                      <PlusCircle className="w-4 h-4" />
                      Add Id
                    </Button>
                  </div>
                ),
              }}
            />
          ),
        }}
      />
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
