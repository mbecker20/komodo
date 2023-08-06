import { ResourceUpdates } from "@components/updates/resource";
import { useAddRecentlyViewed, useRead, useWrite } from "@hooks";
import { ResourceCard } from "@layouts/card";
import { Section } from "@layouts/page";
import { Resource } from "@layouts/resource";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { History } from "lucide-react";
import { AlarmClock, Save, Settings } from "lucide-react";
import { useState } from "react";
import { Link, useParams } from "react-router-dom";

export const AlerterName = ({ id }: { id: string }) => {
  const alerters = useRead("ListAlerters", {}).data;
  const alerter = alerters?.find((a) => a.id === id);
  if (!alerter) return null;
  return <>{alerter.name}</>;
};

const AlerterInfo = ({ id }: { id: string }) => {
  const alerters = useRead("ListAlerters", {}).data;
  const alerter = alerters?.find((a) => a.id === id);
  if (!alerter) return null;
  return <>some description</>;
};

export const AlerterCard = ({ id }: { id: string }) => {
  const alerters = useRead("ListAlerters", {}).data;
  const alerter = alerters?.find((a) => a.id === id);
  if (!alerter) return null;
  return (
    <Link to={`/alerters/${id}`}>
      <ResourceCard
        title={alerter.name}
        description={`${alerter.alerter_type} alerter`}
        statusIcon={<AlarmClock className="w-4 h-4" />}
      >
        <div></div>
      </ResourceCard>
    </Link>
  );
};

export const AlerterConfig = ({ id }: { id: string }) => {
  const alerter = useRead("GetAlerter", { id }).data;
  const [update, set] = useState<Partial<Types.AlerterConfig>>({});
  const { mutate } = useWrite("UpdateAlerter");

  if (!id || !alerter?.config) return null;
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
            onClick={() => {
              if (!update.type || !update.params) return null;
              mutate({
                config: { type: update.type, params: update.params },
                id,
              });
            }}
          >
            <Save className="w-4 h-4" />
          </Button>
        </div>
      }
    >
      {/* <Config config={server?.config as any} update={update} set={set} /> */}
      {/* <Configuration
        config={
          alerter.config as Extract<Types.AlerterConfig, { type: "Slack" }>
        }
        loading={isLoading}
        update={update}
        set={(input) => set((update) => ({ ...update, ...input }))}
        layout={{
          general: ["type", "params"],
        }}
        // overrides={{
        //   params:
        // }}
      /> */}
    </Section>
  );
};

export const AlerterPage = () => {
  const id = useParams().alerterId;
  if (!id) return null;
  useAddRecentlyViewed("Alerter", id);

  return (
    <Resource
      title={<AlerterName id={id} />}
      info={<AlerterInfo id={id} />}
      actions={<></>}
    >
      <ResourceUpdates type="Alerter" id={id} />
      <AlerterConfig id={id} />
    </Resource>
  );
};
