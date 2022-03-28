import { CommandLogError, Network, Server } from "@monitor/types";
import { generateQuery } from "@monitor/util";
import axios from "axios";

export async function getPeripheryNetworks({ address, passkey }: Server) {
  return await axios
    .get<Network[]>(`${address}/networks`, {
      headers: {
        Authorization: passkey,
      },
    })
    .then(({ data }) => data);
}

export async function prunePeripheryNetworks({ address, passkey }: Server) {
  return await axios
    .get<CommandLogError>(`${address}/networks/prune`, {
      headers: {
        Authorization: passkey,
      },
    })
    .then(({ data }) => data);
}

export async function createPeripheryNetwork({ address, passkey }: Server, name: string, driver?: string) {
  return await axios
    .get<CommandLogError>(`${address}/network/create/${name}${generateQuery({ driver })}`, {
      headers: {
        Authorization: passkey,
      },
    })
    .then(({ data }) => data);
}

export async function deletePeripheryNetwork(
  { address, passkey }: Server,
  name: string
) {
  return await axios
    .get<CommandLogError>(`${address}/network/delete/${name}`, {
      headers: {
        Authorization: passkey,
      },
    })
    .then(({ data }) => data);
}
