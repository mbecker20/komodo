import { useInvalidate } from "@hooks";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { toast } from "@ui/toast/use-toast";
import { atom, useAtom } from "jotai";
import { Circle } from "lucide-react";
import { ReactNode, useEffect } from "react";
import rws from "reconnecting-websocket";
import { cn } from "./helpers";
import { UPDATE_WS_URL } from "@main";

const rws_atom = atom(new rws(UPDATE_WS_URL));
const useWebsocket = () => useAtom(rws_atom);

const on_message = (
  { data }: MessageEvent,
  invalidate: ReturnType<typeof useInvalidate>
) => {
  if (data == "LOGGED_IN") return console.log("logged in to ws");
  const update = JSON.parse(data) as Types.UpdateListItem;

  toast({
    title: update.operation,
    description: update.username,
  });

  invalidate(["ListUpdates"]);

  setTimeout(() => {
    if (update.target.type === "Deployment") {
      invalidate(
        ["ListDeployments"],
        ["GetDeployment", { id: update.target.id }],
        ["GetLog", { id: update.target.id }],
        ["GetDeploymentActionState", { id: update.target.id }],
        ["GetDeploymentStatus", { id: update.target.id }]
      );
    }

    if (update.target.type === "Server") {
      invalidate(
        ["ListServers"],
        ["GetServer", { id: update.target.id }],
        ["GetServerActionState", { id: update.target.id }],
        ["GetServerStatus", { id: update.target.id }],
        ["GetHistoricalServerStats", { id: update.target.id }]
      );
    }

    if (update.target.type === "Build") {
      invalidate(
        ["ListBuilds"],
        ["GetBuild", { id: update.target.id }],
        ["GetBuildActionState", { id: update.target.id }]
      );
    }
  }, 100);
};

const on_open = (ws: rws) => {
  const token = localStorage.getItem("monitor-auth-token");
  if (token) ws.send(token);
};

export const WebsocketProvider = ({ children }: { children: ReactNode }) => {
  const invalidate = useInvalidate();
  const [ws] = useWebsocket();

  const on_open_fn = () => on_open(ws);
  const on_message_fn = (e: MessageEvent) => on_message(e, invalidate);

  useEffect(() => {
    ws.addEventListener("open", on_open_fn);
    ws.addEventListener("message", on_message_fn);
    return () => {
      ws.removeEventListener("open", on_open_fn);
      ws.removeEventListener("message", on_message_fn);
    };
  });

  return <>{children}</>;
};

export const WsStatusIndicator = () => {
  const [ws] = useWebsocket();
  const onclick = () =>
    toast({ title: "surprise", description: "motherfucker" });
  return (
    <Button variant="ghost" onClick={onclick}>
      <Circle
        className={cn(
          "w-4 h-4 stroke-none",
          !!ws ? "fill-green-500" : "fill-red-500"
        )}
      />
    </Button>
  );
};
