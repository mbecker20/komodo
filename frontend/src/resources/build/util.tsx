import { useRead } from "@hooks";
import { version_to_string } from "@util/helpers";

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
