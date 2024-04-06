import { Config } from "@components/config";
import { ActionWithDialog } from "@components/util";
import { useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { Trash } from "lucide-react";
import { useState } from "react";
import { useNavigate } from "react-router-dom";

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

export const DeleteAlerter = ({ id }: { id: string }) => {
  const nav = useNavigate();
  const { data: alerter } = useRead("GetAlerter", { alerter: id });
  const { mutateAsync, isPending } = useWrite("DeleteAlerter");

  if (!alerter) return null;
  return (
    <div className="flex items-center justify-between">
      <div className="w-full">Delete Alerter</div>
      <ActionWithDialog
        name={alerter.name}
        title="Delete"
        icon={<Trash className="h-4 w-4" />}
        onClick={async () => {
          await mutateAsync({ id });
          nav("/");
        }}
        disabled={isPending}
        loading={isPending}
      />
    </div>
  );
};
