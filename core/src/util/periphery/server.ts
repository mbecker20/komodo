import { CommandLogError, Server } from "@monitor/types";
import axios from "axios";

export async function prunePeripheryImages({ address, passkey }: Server) {
  return await axios
    .get<CommandLogError>(`http://${address}/images/prune`, {
      headers: {
        Authorization: passkey,
      },
    })
    .then(({ data }) => data);
}
