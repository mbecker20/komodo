import { CommandLogError, Server } from "@monitor/types";
import axios from "axios";
import { SECRETS } from "../../config";

export async function prunePeripheryImages({ address }: Server) {
  return await axios
    .get<CommandLogError>(`${address}/images/prune`, {
      headers: {
        Authorization: SECRETS.PASSKEY,
      },
    })
    .then(({ data }) => data);
}

export async function getPeripheryDockerStats({ address }: Server) {
  return await axios
    .get<CommandLogError>(`${address}/stats`, {
      headers: {
        Authorization: SECRETS.PASSKEY,
      },
    })
    .then(({ data }) => data);
}
