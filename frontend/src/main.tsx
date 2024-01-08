import "globals.css";
import React from "react";
import ReactDOM from "react-dom/client";
import { MonitorClient } from "@monitor/client";
import { ThemeProvider } from "@ui/theme";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { Router } from "@router";
import { WebsocketProvider } from "@lib/socket";
import { Toaster } from "@ui/toaster";

export const MONITOR_BASE_URL =
  import.meta.env.VITE_MONITOR_HOST ?? "https://v1.api.monitor.dev";

export const UPDATE_WS_URL =
  MONITOR_BASE_URL.replace("http", "ws") + "/ws/update";

const token = localStorage.getItem("monitor-auth-token");
export const client = MonitorClient(MONITOR_BASE_URL, token ?? undefined);

const query_client = new QueryClient({
  defaultOptions: { queries: { retry: false } },
});

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <QueryClientProvider client={query_client}>
      <WebsocketProvider url={UPDATE_WS_URL}>
        <ThemeProvider>
          <Router />
          <Toaster />
        </ThemeProvider>
      </WebsocketProvider>
    </QueryClientProvider>
  </React.StrictMode>
);
