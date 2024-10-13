import { DockerContainersSection } from "@components/util";
import { useRead } from "@lib/hooks";

export const Containers = ({
  id,
  show,
  setShow,
}: {
  id: string;
  show: boolean;
  setShow: (show: boolean) => void;
}) => {
  const containers =
    useRead("ListDockerContainers", { server: id }, { refetchInterval: 10_000 })
      .data ?? [];
  return (
    <DockerContainersSection
      server_id={id}
      containers={containers}
      show={show}
      setShow={setShow}
      pruneButton
    />
  );
};
