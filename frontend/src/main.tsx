import React from "react";
import ReactDOM from "react-dom/client";
import "./globals.css";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { MonitorClient } from "@monitor/client";

export const MONITOR_BASE_URL =
  import.meta.env.VITE_MONITOR_HOST ?? "https://v1.api.monitor.mogh.tech";

export const UPDATE_WS_URL =
  MONITOR_BASE_URL.replace("http", "ws") + "/ws/update";

const token = localStorage.getItem("monitor-auth-token");
export const client = MonitorClient(MONITOR_BASE_URL, token ?? undefined);
const queryClient = new QueryClient({
  defaultOptions: { queries: { retry: false } },
});

// eslint-disable-next-line react-refresh/only-export-components
const Router = React.lazy(() => import("./router"));

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <QueryClientProvider client={queryClient}>
      <Router />
    </QueryClientProvider>
  </React.StrictMode>
);
