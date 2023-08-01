import { Config } from "@components/config/Config";
import { useRead, useWrite } from "@hooks";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { Settings, Save, History } from "lucide-react";
import { useState } from "react";
import { useParams } from "react-router-dom";

export const DeploymentConfig = () => {
  const id = useParams().deploymentId;
  const deployment = useRead("GetDeployment", { id }).data;
  const [config, set] = useState<Partial<Types.DeploymentConfig>>({});

  const { mutate } = useWrite("UpdateDeployment");

  if (id && deployment?.config) {
    return (
      <div className="flex flex-col">
        <div className="flex justify-between">
          <div className="flex items-center gap-2 text-muted-foreground">
            <Settings className="w-4 h-4" />
            <h2 className="text-xl">Config</h2>
          </div>
          <div className="flex gap-4">
            <Button variant="outline" intent="warning">
              <History className="w-4 h-4" />
            </Button>
            <Button
              variant="outline"
              intent="success"
              onClick={() => mutate({ config, id })}
            >
              <Save className="w-4 h-4" />
            </Button>
          </div>
        </div>
        <div className="mt-2">
          <Config
            config={deployment?.config as any}
            update={config}
            set={set}
          />
        </div>
      </div>
    );
  } else {
    // loading
    return null;
  }
};
