import { useInvalidate } from "@lib/hooks";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { toast } from "@ui/use-toast";
import { atom, useAtom } from "jotai";
import { Circle } from "lucide-react";
import { ReactNode, useCallback, useEffect } from "react";
import Rws from "reconnecting-websocket";
import { cn } from "@lib/utils";
import { AUTH_TOKEN_STORAGE_KEY } from "@main";

const rws_atom = atom<Rws | null>(null);
const useWebsocket = () => useAtom(rws_atom);

const ws_connected = atom(false);
export const useWebsocketConnected = () => useAtom(ws_connected);

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

const on_open = (ws: Rws | null) => {
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
  const setConnected = useWebsocketConnected()[1];

  const on_open_fn = useCallback(() => {
    on_open(ws);
    setConnected(true);
  }, [ws, setConnected]);
  const on_message_fn = useCallback(
    (e: MessageEvent) => on_message(e, invalidate),
    [invalidate]
  );
  const on_close_fn = useCallback(() => {
    setConnected(false);
  }, [setConnected]);

  useEffect(() => {
    if (!ws) set(new Rws(url));
    return () => {
      ws?.close();
    };
  }, [set, url, ws]);

  useEffect(() => {
    ws?.addEventListener("open", on_open_fn);
    ws?.addEventListener("message", on_message_fn);
    ws?.addEventListener("close", on_close_fn);
    return () => {
      ws?.removeEventListener("open", on_open_fn);
      ws?.removeEventListener("message", on_message_fn);
      ws?.removeEventListener("close", on_close_fn);
    };
  }, [on_message_fn, on_open_fn, on_close_fn, ws]);

  return <>{children}</>;
};

export const WsStatusIndicator = () => {
  const [connected] = useWebsocketConnected();
  const onclick = () =>
    toast({ title: connected ? "Websocket connected" : "Websocket disconnected" });

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
          connected ? "fill-green-500" : "fill-red-500"
        )}
      />
    </Button>
  );
};
