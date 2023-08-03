import { Config } from "@components/config/Config";
import { useRead, useWrite } from "@hooks";
import { Section } from "@layouts/page";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { Settings, Save, History } from "lucide-react";
import { useState } from "react";
import { useParams } from "react-router-dom";

export const BuildConfig = () => {
  const id = useParams().buildId;
  const build = useRead("GetBuild", { id }).data;
  const [update, set] = useState<Partial<Types.BuildConfig>>({});
  const { mutate } = useWrite("UpdateBuild");

  if (id && build?.config) {
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
        <Config config={build?.config as any} update={update} set={set} />
      </Section>
    );
  } else {
    // loading
    return null;
  }
};
