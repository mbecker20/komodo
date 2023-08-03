import { Config } from "@components/config/Config";
import { useRead, useWrite } from "@hooks";
import { Section } from "@layouts/page";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { Settings, Save, History } from "lucide-react";
import { useState } from "react";
import { useParams } from "react-router-dom";

export const BuilderConfig = () => {
  const id = useParams().builderId;
  const deployment = useRead("GetBuilder", { id }).data;
  const [update, set] = useState<Partial<Types.BuilderConfig>>({});
  const { mutate } = useWrite("UpdateBuilder");

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
              onClick={() =>
                mutate({
                  config: { type: "Aws", params: { ...update.params } }, // typecheck angry unless do this
                  id,
                })
              }
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
