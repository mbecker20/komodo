import { useInvalidate, useUser } from "@lib/hooks";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { toast } from "@ui/use-toast";
import { atom, useAtom } from "jotai";
import { Circle } from "lucide-react";
import { ReactNode, useCallback, useEffect, useState } from "react";
import { cn } from "@lib/utils";
import { AUTH_TOKEN_STORAGE_KEY } from "@main";

const rws_atom = atom<WebSocket | null>(null);
const useWebsocket = () => useAtom(rws_atom);

const ws_connected = atom(false);
export const useWebsocketConnected = () => useAtom(ws_connected);

const onMessageHandlers: {
  [key: string]: (update: Types.UpdateListItem) => void;
} = {};

export const useWebsocketMessages = (
  key: string,
  handler: (update: Types.UpdateListItem) => void
) => {
  onMessageHandlers[key] = handler;
  useEffect(() => {
    // Clean up on unmount
    return () => {
      delete onMessageHandlers[key];
    };
  }, []);
};

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
      ["GetDeploymentContainer"],
      ["GetDeploymentsSummary"]
    );
  }

  if (update.target.type === "Server") {
    invalidate(
      ["ListServers"],
      ["GetServer"],
      ["GetServerActionState"],
      ["GetServerState"],
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

  if (update.target.type === "ServerTemplate") {
    invalidate(
      ["ListServerTemplates"],
      ["GetServerTemplate"],
      ["GetServerTemplatesSummary"]
    );
  }

  if (
    update.target.type === "System" &&
    update.operation.includes("Variable")
  ) {
    invalidate(["ListVariables"], ["GetVariable"]);
  }

  // Run any attached handlers
  Object.values(onMessageHandlers).forEach((handler) => handler(update));
};

export const WebsocketProvider = ({
  url,
  children,
}: {
  url: string;
  children: ReactNode;
}) => {
  const user = useUser().data;
  const invalidate = useInvalidate();
  const [ws, set] = useWebsocket();
  const [connected, setConnected] = useWebsocketConnected();
  // don't care about value, just use to make sure value changes
  // to trigger connection useEffect
  const [reconnect, setReconnect] = useState(false);

  const on_message_fn = useCallback(
    (e: MessageEvent) => on_message(e, invalidate),
    [invalidate]
  );

  // Connection useEffect
  useEffect(() => {
    if (user && !connected) {
      const ws = make_websocket({
        url,
        on_open: () => setConnected(true),
        on_message: on_message_fn,
        on_close: () => {
          setConnected(false);
        },
      });
      set(ws);
    }
  }, [set, url, user, connected, reconnect]);

  useEffect(() => {
    // poll for CLOSED state.
    // trigger reconnect after stale page
    const interval = setInterval(() => {
      if (ws?.CLOSED) {
        setConnected(false);
        // toggle to make sure connection useEffect runs.
        // which could happen if connected is stuck in false state,
        // so setConnected(false) doesn't trigger reconnect
        setReconnect(!reconnect);
      }
    }, 3_000);

    return () => clearInterval(interval);
  }, []);

  return <>{children}</>;
};

export const WsStatusIndicator = () => {
  const [connected] = useWebsocketConnected();
  const onclick = () =>
    toast({
      title: connected ? "Websocket connected" : "Websocket disconnected",
    });

  return (
    <Button
      variant="ghost"
      onClick={onclick}
      size="icon"
      className="inline-flex"
    >
      <Circle
        className={cn(
          "w-4 h-4 stroke-none transition-colors",
          connected ? "fill-green-500" : "fill-red-500"
        )}
      />
    </Button>
  );
};

const make_websocket = ({
  url,
  on_open,
  on_message,
  on_close,
}: {
  url: string;
  on_open: () => void;
  on_message: (e: MessageEvent) => void;
  on_close: () => void;
}) => {
  console.log("init websocket");
  const ws = new WebSocket(url);

  const _on_open = () => {
    const jwt = localStorage.getItem(AUTH_TOKEN_STORAGE_KEY);
    if (!ws || !jwt) return;
    const msg: Types.WsLoginMessage = { type: "Jwt", params: { jwt } };
    if (jwt && ws) ws.send(JSON.stringify(msg));
    on_open();
  };

  ws.addEventListener("open", _on_open);
  ws.addEventListener("message", on_message);
  ws.addEventListener("close", on_close);

  return ws;
};
