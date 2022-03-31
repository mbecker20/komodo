import { Collection, CommandLogError, ContainerStatus, Log, Server } from "@monitor/types";
import { generateQuery } from "@monitor/util";
import axios from "axios";
import { SECRETS } from "../../config";

export async function getPeripheryContainers({ address }: Server) {
  return (await axios.get(`${address}/containers`, {
    headers: {
      Authorization: SECRETS.PASSKEY,
    },
  }).then(({ data }) => data)) as Collection<ContainerStatus>;
}

export async function getPeripheryContainer(
  { address }: Server,
  name: string
) {
	return (await axios
    .get(`${address}/container/${name}`, {
      headers: {
        Authorization: SECRETS.PASSKEY,
      },
    })
    .then(({ data }) => data)) as ContainerStatus | "not deployed";
}

export async function getPeripheryContainerLog(
  { address }: Server,
  name: string,
  tail?: number
) {
  return (await axios
    .get(`${address}/container/log/${name}${generateQuery({ tail })}`, {
      headers: {
        Authorization:  SECRETS.PASSKEY,
      },

    })
    .then(({ data }) => data)) as Log;
}

export async function startPeripheryContainer(
  { address }: Server,
  name: string
) {
  return (await axios
    .get(`${address}/container/start/${name}`, {
      headers: {
        Authorization:  SECRETS.PASSKEY,
      },
    })
    .then(({ data }) => data)) as CommandLogError;
}

export async function stopPeripheryContainer(
  { address }: Server,
  name: string
) {
  return (await axios
    .get(`${address}/container/stop/${name}`, {
      headers: {
        Authorization:  SECRETS.PASSKEY,
      },
    })
    .then(({ data }) => data)) as CommandLogError;
}

export async function deletePeripheryContainer(
  { address }: Server,
  name: string
) {
  return (await axios
    .get(`${address}/container/delete/${name}`, {
      headers: {
        Authorization:  SECRETS.PASSKEY,
      },
    })
    .then(({ data }) => data)) as CommandLogError;
}


