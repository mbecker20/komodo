import { useRead } from "@hooks";
import { version_to_string } from "@util/helpers";
import { Factory, History } from "lucide-react";

export const BuildName = ({ id }: { id: string | undefined }) => {
  const builds = useRead("ListBuilds", {}).data;
  const build = builds?.find((b) => b.id === id);
  return <>{build?.name ?? "..."}</>;
};

export const BuildVersion = ({ id }: { id: string }) => {
  const builds = useRead("ListBuilds", {}).data;
  const build = builds?.find((b) => b.id === id);
  return <>{version_to_string(build?.info.version) ?? "..."}</>;
};

export const BuildBuilder = ({ id }: { id: string }) => {
  const builds = useRead("ListBuilds", {}).data;
  const build = builds?.find((b) => b.id === id);
  return <>{build?.id ?? "..."}</>;
};

export const BuildLastBuilt = ({ id }: { id: string }) => {
  const builds = useRead("ListBuilds", {}).data;
  const build = builds?.find((b) => b.id === id);
  const last = build?.info.last_built_at;
  return <>{last ? new Date(last).toLocaleString() : "not yet built"}</>;
};

export const BuildInfo = ({ id }: { id: string }) => {
  return (
    <div className="flex flex-col text-muted-foreground text-sm">
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
