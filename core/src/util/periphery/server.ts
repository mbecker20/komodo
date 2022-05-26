import { CommandLogError, DockerStat, PM2Process, Server, SystemStats } from "@monitor/types";
import axios from "axios";
import { SECRETS } from "../../config";

export async function prunePeripheryImages({ address, passkey }: Server) {
  return await axios
    .get<CommandLogError>(`${address}/images/prune`, {
      headers: {
        Authorization: passkey || SECRETS.PASSKEY,
      },
    })
    .then(({ data }) => data);
}

export async function getPeripheryDockerStats({ address, passkey }: Server) {
  return await axios
    .get<DockerStat[]>(`${address}/stats`, {
      headers: {
        Authorization: passkey || SECRETS.PASSKEY,
      },
    })
    .then(({ data }) => data);
}

export async function getPeripherySystemStats({ address, passkey }: Server) {
  return await axios
    .get<SystemStats>(`${address}/sys-stats`, {
      headers: {
        Authorization: passkey || SECRETS.PASSKEY,
      },
    })
    .then(({ data }) => data);
}