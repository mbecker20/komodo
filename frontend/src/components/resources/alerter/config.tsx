import { Config } from "@components/config";
import { useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { useState } from "react";

export const AlerterConfig = ({ id }: { id: string }) => {
  const config = useRead("GetAlerter", { alerter: id }).data?.config;
  if (config?.type === "Slack") return <SlackAlerterConfig id={id} />;
  if (config?.type === "Custom") return <CustomAlerterConfig id={id} />;
};

const SlackAlerterConfig = ({ id }: { id: string }) => {
  const config = useRead("GetAlerter", { alerter: id }).data?.config;
  const [update, set] = useState<Partial<Types.SlackAlerterConfig>>({});
  const { mutate } = useWrite("UpdateAlerter");
  if (!config) return null;

  return (
    <Config
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
  const config = useRead("GetAlerter", { alerter: id }).data?.config;
  const [update, set] = useState<Partial<Types.CustomAlerterConfig>>({});
  const { mutate } = useWrite("UpdateAlerter");
  if (!config) return null;

  return (
    <Config
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
