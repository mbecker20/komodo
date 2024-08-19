import { Section } from "@components/layouts";
import { ReactNode } from "react";
import { Networks } from "./networks";
import { useServer } from "..";
import { Types } from "@monitor/client";

export const ServerInfo = ({
  id,
  titleOther,
}: {
  id: string;
  titleOther: ReactNode;
}) => {
  const state = useServer(id)?.info.state ?? Types.ServerState.NotOk;

  if ([Types.ServerState.NotOk, Types.ServerState.Disabled].includes(state)) {
    return (
      <Section titleOther={titleOther}>
        <h2 className="text-muted-foreground">
          Server unreachable, info is not available
        </h2>
      </Section>
    );
  }

  return (
    <Section titleOther={titleOther}>
      <Networks id={id} />
    </Section>
  );
};
