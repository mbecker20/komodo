import { Config } from "@components/config/Config";
import { useRead, useWrite } from "@hooks";
import { Section } from "@layouts/page";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { Settings, Save, History } from "lucide-react";
import { useState } from "react";
import { useParams } from "react-router-dom";

export const DeploymentConfig = () => {
  const id = useParams().deploymentId;
  const deployment = useRead("GetDeployment", { id }).data;
  const [update, set] = useState<Partial<Types.DeploymentConfig>>({});
  const { mutate } = useWrite("UpdateDeployment");

  if (id && deployment?.config) {
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
        <Config config={deployment?.config as any} update={update} set={set} />
      </Section>
    );
  } else {
    // loading
    return null;
  }
};
