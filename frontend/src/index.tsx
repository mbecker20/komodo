/* @refresh reload */
import { render } from 'solid-js/web';
import App from './App';

export const URL =
  import.meta.env.MODE === "production"
    ? location.origin
    : (import.meta.env.VITE_MONITOR_HOST as string) || "http://localhost:9000";

export const WS_URL = URL.replace("http", "ws") + "/ws";

render(() => <App />, document.getElementById('root') as HTMLElement);
