import { Configuration } from "@components/config";
import { ConfirmUpdate } from "@components/config/confirm-update";
import { ManualConfig } from "@components/config/manual-config";
import { useRead, useWrite } from "@hooks";
import { Section } from "@layouts/page";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { Input } from "@ui/input";
import { Settings, Save, History } from "lucide-react";
import { useState } from "react";
import { useParams } from "react-router-dom";

type ServerUpdate = Partial<Types.ServerConfig>;

export const SerCon = ({ id }: { id: string }) => {
  const config = useRead("GetServer", { id }).data?.config;
  const { mutate } = useWrite("UpdateServer");
  const [update, set] = useState<ServerUpdate>({});
  const up = <K extends keyof Types.ServerConfig, V = Types.ServerConfig[K]>(
    k: K,
    v: V
  ) => set((p) => ({ ...p, [k]: v }));

  if (!config) return null;

  return (
    <Section
      title="Config"
      icon={<Settings className="w-4 h-4" />}
      actions={
        <div className="flex gap-4">
          <Button variant="outline" intent="warning" onClick={() => set({})}>
            <History className="w-4 h-4" />
          </Button>
          <Button
            variant="outline"
            intent="success"
            onClick={() => mutate({ config: update, id })}
          >
            <Save className="w-4 h-4" />
          </Button>
        </div>
      }
    >
      <ManualConfig
        layout={{
          general: {
            components: [
              {
                title: "Address",
                element: (
                  <Input
                    value={update.address ?? config.address}
                    onChange={(e) => up("address", e.target.value)}
                    className="max-w-[400px]"
                  />
                ),
              },
              {
                title: "Region",
                element: (
                  <Input
                    value={update.region ?? config.region}
                    onChange={(e) => up("region", e.target.value)}
                    className="max-w-[400px]"
                  />
                ),
              },
            ],
          },
        }}
      />
    </Section>
  );
};

export const ServerConfig = () => {
  const id = useParams().serverId;
  const server = useRead("GetServer", { id }).data;
  const [update, set] = useState<Partial<Types.ServerConfig>>({});
  const { mutate, isLoading } = useWrite("UpdateServer");

  if (id && server?.config) {
    return (
      <Section
        title="Config"
        icon={<Settings className="w-4 h-4" />}
        actions={
          <div className="flex gap-4">
            <Button variant="outline" intent="warning" onClick={() => set({})}>
              <History className="w-4 h-4" />
            </Button>
            <ConfirmUpdate
              content={JSON.stringify(update, null, 2)}
              onConfirm={() => mutate({ config: update, id })}
            />
          </div>
        }
      >
        {/* <Config config={server?.config as any} update={update} set={set} /> */}
        <Configuration
          config={server.config}
          loading={isLoading}
          update={update}
          set={(input) => set((update) => ({ ...update, ...input }))}
          layout={{
            general: ["address", "region", "enabled", "auto_prune"],
            warnings: [
              "cpu_warning",
              "cpu_critical",
              "disk_warning",
              "disk_critical",
              "mem_warning",
              "mem_critical",
            ],
          }}
        />
      </Section>
    );
  } else {
    // loading
    return null;
  }
};
