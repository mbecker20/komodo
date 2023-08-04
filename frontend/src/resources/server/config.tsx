import { Configuration } from "@components/config";
import { useRead, useWrite } from "@hooks";
import { Section } from "@layouts/page";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { Settings, Save, History } from "lucide-react";
import { useState } from "react";
import { useParams } from "react-router-dom";

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
          // overrides={}
        />
      </Section>
    );
  } else {
    // loading
    return null;
  }
};
