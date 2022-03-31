import { CommandLogError, Network, Server } from "@monitor/types";
import { generateQuery } from "@monitor/util";
import axios from "axios";
import { SECRETS } from "../../config";

export async function getPeripheryNetworks({ address }: Server) {
  return await axios
    .get<Network[]>(`${address}/networks`, {
      headers: {
        Authorization: SECRETS.PASSKEY,
      },
    })
    .then(({ data }) => data);
}

export async function prunePeripheryNetworks({ address }: Server) {
  return await axios
    .get<CommandLogError>(`${address}/networks/prune`, {
      headers: {
        Authorization: SECRETS.PASSKEY,
      },
    })
    .then(({ data }) => data);
}

export async function createPeripheryNetwork({ address }: Server, name: string, driver?: string) {
  return await axios
    .get<CommandLogError>(`${address}/network/create/${name}${generateQuery({ driver })}`, {
      headers: {
        Authorization: SECRETS.PASSKEY,
      },
    })
    .then(({ data }) => data);
}

export async function deletePeripheryNetwork(
  { address }: Server,
  name: string
) {
  return await axios
    .get<CommandLogError>(`${address}/network/delete/${name}`, {
      headers: {
        Authorization: SECRETS.PASSKEY,
      },
    })
    .then(({ data }) => data);
}
