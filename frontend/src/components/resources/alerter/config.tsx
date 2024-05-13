import { Config } from "@components/config";
import { useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";
import { useEffect, useState } from "react";

export const AlerterConfig = ({ id }: { id: string }) => {
  const perms = useRead("GetPermissionLevel", {
    target: { type: "Alerter", id },
  }).data;
  const config = useRead("GetAlerter", { alerter: id }).data?.config;
  const [type, setType] = useState<Types.AlerterConfig["type"]>();
  useEffect(() => config?.type && setType(config.type), [config?.type]);
  const [update, setConfig] = useState<
    Partial<Types.SlackAlerterConfig | Types.CustomAlerterConfig>
  >({});
  const { mutateAsync } = useWrite("UpdateAlerter");
  if (!config) return null;

  const disabled = perms !== Types.PermissionLevel.Write;

  return (
    <Config
      disabled={disabled}
      config={config.params}
      update={update}
      set={setConfig}
      onSave={async () => {
        if (!type) return;
        await mutateAsync({ id, config: { type, params: update } });
      }}
      components={{
        general: [
          {
            label: "General",
            components: {
              url: true,
              enabled: true,
            },
          },
        ],
      }}
      selector={
        <div className="flex gap-2 items-center text-sm">
          Alerter Type:
          <Select
            value={type}
            onValueChange={(type) => {
              setType(type as any);
              setConfig({
                url: update.url || "",
                enabled: update.enabled === undefined ? true : update.enabled,
              });
            }}
          >
            <SelectTrigger className="w-32 capitalize">
              <SelectValue />
            </SelectTrigger>
            <SelectContent className="w-32">
              {["Slack", "Custom"].map((key) => (
                <SelectItem value={key} key={key} className="capitalize">
                  {key}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
      }
    />
  );
};
