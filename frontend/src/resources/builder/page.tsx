import { useRead, useSetRecentlyViewed } from "@hooks";
import { Resource } from "@layouts/resource";
import { useParams } from "react-router-dom";

const BuilderName = ({ id }: { id: string }) => {
  const builders = useRead("ListBuilders", {}).data;
  const builder = builders?.find((b) => b._id?.$oid === id);
  return <>{builder?.name}</>;
};

export const BuilderPage = () => {
  const id = useParams().builderId;
  const push = useSetRecentlyViewed();

  if (!id) return null;
  push("Builder", id);

  return (
    <Resource title={<BuilderName id={id} />} info={<></>} actions={<></>}>
      <div>builder page</div>
    </Resource>
  );
};
