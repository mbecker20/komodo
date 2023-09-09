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
import { ConfigInner } from "@components/config";

const useAlerter = (id?: string) =>
  useRead("ListAlerters", {}).data?.find((d) => d.id === id);

const NewAlerter = () => {
  const { mutateAsync } = useWrite("CreateDeployment");
  const [name, setName] = useState("");
  const [type, setType] = useState<Types.AlerterConfig["type"]>();

  return (
    <NewResource
      type="Alerter"
      onSuccess={() => mutateAsync({ name, config: {} })}
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
  const config = useRead("GetAlerter", { id }).data?.config;
  const [update, set] = useState<Partial<Types.SlackAlerterConfig>>({});
  const { mutate } = useWrite("UpdateAlerter");
  if (!config) return null;

  return (
    <ConfigInner
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
  const config = useRead("GetAlerter", { id }).data?.config;
  const [update, set] = useState<Partial<Types.CustomAlerterConfig>>({});
  const { mutate } = useWrite("UpdateAlerter");
  if (!config) return null;

  return (
    <ConfigInner
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

export const Alerter: RequiredResourceComponents = {
  Name: ({ id }) => <>{useAlerter(id)?.name}</>,
  Description: ({ id }) => <>{useAlerter(id)?.info.alerter_type} alerter</>,
  Info: ({ id }) => <>{id}</>,
  Icon: () => <AlarmClock className="w-4 h-4" />,
  Page: {
    Config: ({ id }: { id: string }) => {
      const config = useRead("GetAlerter", { id }).data?.config;
      if (config?.type === "Slack") return <SlackAlerterConfig id={id} />;
      if (config?.type === "Custom") return <CustomAlerterConfig id={id} />;
    },
  },
  Actions: () => null,
  New: () => <NewAlerter />,
};
