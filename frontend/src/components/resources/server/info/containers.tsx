import { DockerContainersSection } from "@components/util";
import { useRead } from "@lib/hooks";
import { ReactNode } from "react";

export const Containers = ({
  id,
  titleOther
}: {
  id: string;
  titleOther: ReactNode
}) => {
  const containers =
    useRead("ListDockerContainers", { server: id }, { refetchInterval: 10_000 })
      .data ?? [];
  return (
    <DockerContainersSection
      server_id={id}
      containers={containers}
      titleOther={titleOther}
      pruneButton
    />
  );
};
