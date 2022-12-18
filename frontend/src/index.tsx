/* @refresh reload */
import { render } from 'solid-js/web';
import App from './App';
import { Client } from './util/client';

export const URL =
  import.meta.env.MODE === "production"
    ? location.origin
    : (import.meta.env.VITE_MONITOR_HOST as string) || "http://localhost:9000";

export const WS_URL = URL.replace("http", "ws") + "/ws";

const token =
  localStorage.getItem("access_token") ||
  (import.meta.env.VITE_ACCESS_TOKEN as string) ||
  null;

export const client = new Client(URL, token);

render(() => <App />, document.getElementById('root') as HTMLElement);
