import { CommandLogError, Server } from "@monitor/types";
import axios from "axios";

export async function prunePeriphery({ address, passkey }: Server) {
  return await axios
    .get<CommandLogError>(`http://${address}/prune`, {
      headers: {
        Authorization: passkey,
      },
    })
    .then(({ data }) => data);
}
