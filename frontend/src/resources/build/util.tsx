import { useRead } from "@hooks";
import { CardDescription } from "@ui/card";
import { version_to_string } from "@util/helpers";
import { Factory, History } from "lucide-react";

export const BuildName = ({ id }: { id: string }) => {
  const builds = useRead({ type: "ListBuilds", params: {} }).data;
  const build = builds?.find((b) => b.id === id);
  return <>{build?.name ?? "..."}</>;
};

export const BuildVersion = ({ id }: { id: string }) => {
  const builds = useRead({ type: "ListBuilds", params: {} }).data;
  const build = builds?.find((b) => b.id === id);
  return <>{version_to_string(build?.version) ?? "..."}</>;
};

export const BuildBuilder = ({ id }: { id: string }) => {
  const builds = useRead({ type: "ListBuilds", params: {} }).data;
  const build = builds?.find((b) => b.id === id);
  return <>{"build.builder " + build?.id ?? "..."}</>;
};

export const BuildLastBuilt = ({ id }: { id: string }) => {
  const builds = useRead({ type: "ListBuilds", params: {} }).data;
  const build = builds?.find((b) => b.id === id);
  const last = build?.last_built_at;
  return <>{last ? new Date(last).toLocaleString() : "not yet built"}</>;
};

export const BuildInfo = ({ id }: { id: string }) => {
  return (
    <div className="flex flex-col gap-2 md:flex-row md:gap-4">
      <CardDescription className="flex items-center">
        <Factory className="w-4 h-4 mr-2" />
        <BuildBuilder id={id} />
      </CardDescription>
      <CardDescription className="flex items-center">
        <History className="w-4 h-4 mr-2" />
        <BuildLastBuilt id={id} />
      </CardDescription>
    </div>
  );
};
