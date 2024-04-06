import { ServerStats } from "@components/resources/server/stats";
import { useResourceParamType } from "@lib/hooks";
import { useParams } from "react-router-dom";

export const ResourceStats = () => {
  const type = useResourceParamType()!;
  const id = useParams().id as string;

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
