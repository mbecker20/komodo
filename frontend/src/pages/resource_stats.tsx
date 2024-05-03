import { ServerStats } from "@components/resources/server/stats";
import { useRead, useResourceParamType, useSetTitle } from "@lib/hooks";
import { useParams } from "react-router-dom";

export const ResourceStats = () => {
  const type = useResourceParamType()!;
  const id = useParams().id as string;
  const name = useRead(`List${type}s`, {}).data?.find((r) => r.id === id)?.name;
  useSetTitle(name && `${name} | Stats`);

  if (type === "Server") {
    return <ServerStats id={id} />;
  } else {
    return (
      <div className="w-full h-full flex justify-center items-center">
        This page does not exist
      </div>
    );
  }
};
