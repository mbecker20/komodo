import { ResourceUpdates } from "@components/updates/resource";
import { useRead, useWrite } from "@hooks";
import { Resource } from "@layouts/resource";
import { useEffect } from "react";
import { useParams } from "react-router-dom";

const BuilderName = ({ id }: { id: string }) => {
  const builders = useRead("ListBuilders", {}).data;
  const builder = builders?.find((b) => b._id?.$oid === id);
  return <>{builder?.name}</>;
};

export const BuilderPage = () => {
  const id = useParams().builderId;
  const push = useWrite("PushRecentlyViewed").mutate;

  if (!id) return null;
  useEffect(() => {
    push({ resource: { type: "Builder", id } });
  }, []);

  return (
    <Resource title={<BuilderName id={id} />} info={<></>} actions={<></>}>
      <ResourceUpdates type="Builder" id={id} />
      <div>builder page</div>
    </Resource>
  );
};
