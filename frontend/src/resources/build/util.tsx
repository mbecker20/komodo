import { useRead } from "@hooks";
import { CardDescription } from "@ui/card";
import { version_to_string } from "@util/helpers";
import { Factory, History } from "lucide-react";

export const BuildName = ({ id }: { id: string }) => {
  const builds = useRead("ListBuilds", {}).data;
  const build = builds?.find((b) => b.id === id);
  return <>{build?.name ?? "..."}</>;
};

export const BuildVersion = ({ id }: { id: string }) => {
  const builds = useRead("ListBuilds", {}).data;
  const build = builds?.find((b) => b.id === id);
  return <>{version_to_string(build?.version) ?? "..."}</>;
};

export const BuildBuilder = ({ id }: { id: string }) => {
  const builds = useRead("ListBuilds", {}).data;
  const build = builds?.find((b) => b.id === id);
  return <>{build?.id.slice(0, 10) + "..." ?? "..."}</>;
};

export const BuildLastBuilt = ({ id }: { id: string }) => {
  const builds = useRead("ListBuilds", {}).data;
  const build = builds?.find((b) => b.id === id);
  const last = build?.last_built_at;
  return <>{last ? new Date(last).toLocaleString() : "not yet built"}</>;
};

export const BuildInfo = ({ id }: { id: string }) => {
  return (
    <div className="flex items-center gap-4 text-sm text-muted-foreground">
      <div className="flex items-center gap-2">
        <Factory className="w-4 h-4" />
        <BuildBuilder id={id} />
      </div>
      <div className="flex items-center gap-2">
        <History className="w-4 h-4" />
        <BuildLastBuilt id={id} />
      </div>
    </div>
  );
};
