import { Types } from "@monitor/client";
import { toast } from "@ui/toast/use-toast";
import { useState } from "react";
import rws from "reconnecting-websocket";

export const WebsocketProvider = () => {
  const ws = new rws("ws-url");
  const [open, set] = useState(false);

  ws.addEventListener("open", () => {
    const token = localStorage.getItem("token");
    if (token) ws.send(token);
    set(true);
  });

  ws.addEventListener("message", ({ data }) => {
    if (data == "LOGGED_IN") return console.log("logged in to ws");
    const update = JSON.parse(data) as Types.Update;
    toast({
      title: `${update.target} ${update.operation}`,
      description: update.operator,
    });
  });
};
