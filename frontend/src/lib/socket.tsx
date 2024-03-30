import { useInvalidate } from "@lib/hooks";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { toast } from "@ui/use-toast";
import { atom, useAtom } from "jotai";
import { Circle } from "lucide-react";
import { ReactNode, useCallback, useEffect } from "react";
import rws from "reconnecting-websocket";
import { cn } from "@lib/utils";
import { AUTH_TOKEN_STORAGE_KEY } from "@main";

const rws_atom = atom<rws | null>(null);
const useWebsocket = () => useAtom(rws_atom);

const on_message = (
  { data }: MessageEvent,
  invalidate: ReturnType<typeof useInvalidate>
) => {
  if (data == "LOGGED_IN") return console.info("logged in to ws");
  const update = JSON.parse(data) as Types.UpdateListItem;

  toast({
    title: update.operation,
    description: update.username,
  });

  invalidate(["ListUpdates"]);
  invalidate(["GetUpdate", { id: update.id }]);

  if (update.target.type === "Deployment") {
    invalidate(
      ["ListDeployments"],
      ["GetDeployment"],
      ["GetLog"],
      ["GetDeploymentActionState"],
      ["GetDeploymentStatus"],
      ["GetDeploymentsSummary"]
    );
  }

  if (update.target.type === "Server") {
    invalidate(
      ["ListServers"],
      ["GetServer"],
      ["GetServerActionState"],
      ["GetServerStatus"],
      ["GetHistoricalServerStats"],
      ["GetServersSummary"]
    );
  }

  if (update.target.type === "Build") {
    invalidate(
      ["ListBuilds"],
      ["GetBuild"],
      ["GetBuildActionState"],
      ["GetBuildMonthlyStats"],
      ["GetBuildVersions"],
      ["GetBuildsSummary"]
    );
  }

  if (update.target.type === "Repo") {
    invalidate(
      ["ListRepos"],
      ["GetRepo"],
      ["GetRepoActionState"],
      ["GetReposSummary"]
    );
  }

  if (update.target.type === "Procedure") {
    invalidate(
      ["ListProcedures"],
      ["GetProcedure"],
      ["GetProcedureActionState"],
      ["GetProceduresSummary"]
    );
  }

  if (update.target.type === "Builder") {
    invalidate(
      ["ListBuilders"],
      ["GetBuilder"],
      ["GetBuilderAvailableAccounts"],
      ["GetBuildersSummary"]
    );
  }

  if (update.target.type === "Alerter") {
    invalidate(["ListAlerters"], ["GetAlerter"], ["GetAlertersSummary"]);
  }
};

const on_open = (ws: rws | null) => {
  const jwt = localStorage.getItem(AUTH_TOKEN_STORAGE_KEY);
  if (!ws || !jwt) return;
  const msg: Types.WsLoginMessage = { type: "Jwt", params: { jwt } };
  if (jwt && ws) ws.send(JSON.stringify(msg));
};

export const WebsocketProvider = ({
  url,
  children,
}: {
  url: string;
  children: ReactNode;
}) => {
  const invalidate = useInvalidate();
  const [ws, set] = useWebsocket();

  const on_open_fn = useCallback(() => on_open(ws), [ws]);
  const on_message_fn = useCallback(
    (e: MessageEvent) => on_message(e, invalidate),
    [invalidate]
  );

  useEffect(() => {
    if (!ws) set(new rws(url));
    return () => {
      ws?.close();
    };
  }, [set, url, ws]);

  useEffect(() => {
    ws?.addEventListener("open", on_open_fn);
    ws?.addEventListener("message", on_message_fn);
    return () => {
      ws?.close();
      ws?.removeEventListener("open", on_open_fn);
      ws?.removeEventListener("message", on_message_fn);
    };
  }, [on_message_fn, on_open_fn, ws]);

  return <>{children}</>;
};

export const WsStatusIndicator = () => {
  const [ws] = useWebsocket();
  const onclick = () =>
    toast({ title: "surprise", description: "motherfucker" });

  return (
    <Button
      variant="ghost"
      onClick={onclick}
      size="icon"
      className="hidden md:inline-flex"
    >
      <Circle
        className={cn(
          "w-4 h-4 stroke-none",
          ws ? "fill-green-500" : "fill-red-500"
        )}
      />
    </Button>
  );
};
