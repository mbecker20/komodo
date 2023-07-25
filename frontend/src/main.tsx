import React from "react";
import ReactDOM from "react-dom/client";
import "./globals.css";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { MonitorClient } from "@monitor/client";
// import { monitor_client } from "@util/client.ts";

export const MONITOR_BASE_URL =
  import.meta.env.VITE_MONITOR_HOST ?? "https://monitor.v1.api.mogh.tech";

export const UPDATE_WS_URL =
  MONITOR_BASE_URL.replace("http", "ws") + "/ws/update";

// const token =
//   (import.meta.env.VITE_ACCESS_TOKEN as string) ||
//   localStorage.getItem("access_token") ||
//   null;

// export const client = monitor_client(MONITOR_BASE_URL, token);
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
