import { ResourceCard } from "@layouts/card";
import { Bot, Cloud, Factory } from "lucide-react";
import { ResourceUpdates } from "@components/updates/resource";
import { useRead, useWrite } from "@hooks";
import { Resource } from "@layouts/resource";
import { useEffect } from "react";
import { Link, useParams } from "react-router-dom";

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

export const BuilderCard = ({ id }: { id: string }) => {
  const builders = useRead("ListBuilders", {}).data;
  const builder = builders?.find((b) => b._id?.$oid === id);
  if (!builder) return null;
  return (
    <Link to={`/builders/${builder._id?.$oid}`}>
      <ResourceCard
        title={builder.name}
        description={"some description"}
        statusIcon={<Factory className="w-4 h-4" />}
      >
        <div className="flex flex-col text-muted-foreground text-sm">
          <div className="flex items-center gap-2">
            <Cloud className="w-4 h-4" />
            AWS
          </div>
          <div className="flex items-center gap-2">
            <Bot className="w-4 h-4" />
            C5x Large
          </div>
        </div>
      </ResourceCard>
    </Link>
  );
};
