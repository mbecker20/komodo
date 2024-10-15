import { Section } from "@components/layouts";
import { ReactNode } from "react";
import { Networks } from "./networks";
import { useServer } from "..";
import { Types } from "komodo_client";
import { useLocalStorage } from "@lib/hooks";
import { Images } from "./images";
import { Containers } from "./containers";
import { Volumes } from "./volumes";
import { Button } from "@ui/button";

export const ServerInfo = ({
  id,
  titleOther,
}: {
  id: string;
  titleOther: ReactNode;
}) => {
  const state = useServer(id)?.info.state ?? Types.ServerState.NotOk;
  const [show, setShow] = useLocalStorage<{
    containers: boolean;
    networks: boolean;
    images: boolean;
    volumes: boolean;
  }>("server-info-show-config", {
    containers: true,
    networks: true,
    images: true,
    volumes: true,
  });

  if ([Types.ServerState.NotOk, Types.ServerState.Disabled].includes(state)) {
    return (
      <Section titleOther={titleOther}>
        <h2 className="text-muted-foreground">
          Server unreachable, info is not available
        </h2>
      </Section>
    );
  }

  const anyOpen = !Object.values(show).every((val) => !val);

  return (
    <Section
      titleOther={titleOther}
      actions={
        <Button
          size="sm"
          variant="outline"
          onClick={() =>
            setShow({
              containers: !anyOpen,
              networks: !anyOpen,
              images: !anyOpen,
              volumes: !anyOpen,
            })
          }
        >
          {anyOpen ? "Hide All" : "Show All"}
        </Button>
      }
    >
      <div className="flex flex-col gap-4">
        <Containers
          id={id}
          show={show.containers}
          setShow={(containers) => setShow({ ...show, containers })}
        />
        <Networks
          id={id}
          show={show.networks}
          setShow={(networks) => setShow({ ...show, networks })}
        />
        <Volumes
          id={id}
          show={show.volumes}
          setShow={(volumes) => setShow({ ...show, volumes })}
        />
        <Images
          id={id}
          show={show.images}
          setShow={(images) => setShow({ ...show, images })}
        />
      </div>
    </Section>
  );
};
