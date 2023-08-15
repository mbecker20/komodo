import { useRead, useWrite } from "@hooks";
import { Section } from "@layouts/page";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { Settings, Save, History } from "lucide-react";
import { useState } from "react";
import { useParams } from "react-router-dom";

export const AlerterConfig = () => {
  const id = useParams().builderId;
  const alerter = useRead("GetAlerter", { id }).data;
  const [update, set] = useState<Partial<Types.AlerterConfig>>({});
  const { mutate } = useWrite("UpdateAlerter");

  if (id && alerter?.config) {
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
                  config: { type: "Slack", params: { ...update.params } }, // typecheck angry unless do this
                  id,
                })
              }
            >
              <Save className="w-4 h-4" />
            </Button>
          </div>
        }
      >
        {/* <Config config={alerter?.config as any} update={update} set={set} /> */}
      </Section>
    );
  } else {
    // loading
    return null;
  }
};
