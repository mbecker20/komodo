import { ResourceUpdates } from "@components/updates/resource";
import { useAddRecentlyViewed, useRead } from "@hooks";
import { ResourceCard } from "@layouts/card";
import { Resource } from "@layouts/resource";
import { AlarmClock } from "lucide-react";
import { Link, useParams } from "react-router-dom";

export const AlerterName = ({ id }: { id: string }) => {
  const alerters = useRead("ListAlerters", {}).data;
  const alerter = alerters?.find((a) => a._id?.$oid === id);
  if (!alerter) return null;
  return <>{alerter.name}</>;
};

const AlerterInfo = ({ id }: { id: string }) => {
  const alerters = useRead("ListAlerters", {}).data;
  const alerter = alerters?.find((a) => a._id?.$oid === id);
  if (!alerter) return null;
  return <>some description</>;
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
    </Resource>
  );
};

export const AlerterCard = ({ id }: { id: string }) => {
  const alerters = useRead("ListAlerters", {}).data;
  const alerter = alerters?.find((a) => a._id?.$oid === id);
  if (!alerter) return null;
  return (
    <Link to={`/alerters/${id}`}>
      <ResourceCard
        title={alerter.name}
        description={alerter.description ?? "some desc"}
        statusIcon={<AlarmClock className="w-4 h-4" />}
      >
        <div></div>
      </ResourceCard>
    </Link>
  );
};
