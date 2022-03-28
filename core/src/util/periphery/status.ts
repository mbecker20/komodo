import { Server } from "@monitor/types";
import axios from "axios";
import { SERVER_CHECK_TIMEOUT } from "../../config";

export async function serverStatusPeriphery({
  address,
  passkey,
  enabled,
  isCore,
}: Server) {
  // returns true if can be reached, false else
  if (isCore) return true;
  if (!enabled) return false;

  const controller = new AbortController();
  const timeout = setTimeout(() => {
    controller.abort();
  }, SERVER_CHECK_TIMEOUT);

  try {
    await axios.get(`${address}/status`, {
      headers: {
        Authorization: passkey,
      },
      signal: controller.signal,
    });
    return true;
  } catch (error) {
    return false;
  } finally {
    clearTimeout(timeout);
  }
}