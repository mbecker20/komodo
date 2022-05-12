import {
  Collection,
  CommandLogError,
  ContainerStatus,
  Log,
  Server,
} from "@monitor/types";
import { generateQuery } from "@monitor/util";
import axios from "axios";
import { SECRETS } from "../../config";

export async function getPeripheryContainers({ address, passkey }: Server) {
  return (await axios
    .get(`${address}/containers`, {
      headers: {
        Authorization: passkey || SECRETS.PASSKEY,
      },
    })
    .then(({ data }) => data)) as Collection<ContainerStatus>;
}

export async function getPeripheryContainer(
  { address, passkey }: Server,
  name: string
) {
  return (await axios
    .get(`${address}/container/${name}`, {
      headers: {
        Authorization: passkey || SECRETS.PASSKEY,
      },
    })
    .then(({ data }) => data)) as ContainerStatus | "not deployed";
}

export async function getPeripheryContainerLog(
  { address, passkey }: Server,
  name: string,
  tail?: number
) {
  return (await axios
    .get(`${address}/container/log/${name}${generateQuery({ tail })}`, {
      headers: {
        Authorization: passkey || SECRETS.PASSKEY,
      },
    })
    .then(({ data }) => data)) as Log;
}

export async function startPeripheryContainer(
  { address, passkey }: Server,
  name: string
) {
  return (await axios
    .get(`${address}/container/start/${name}`, {
      headers: {
        Authorization: passkey || SECRETS.PASSKEY,
      },
    })
    .then(({ data }) => data)) as CommandLogError;
}

export async function stopPeripheryContainer(
  { address, passkey }: Server,
  name: string
) {
  return (await axios
    .get(`${address}/container/stop/${name}`, {
      headers: {
        Authorization: passkey || SECRETS.PASSKEY,
      },
    })
    .then(({ data }) => data)) as CommandLogError;
}

export async function deletePeripheryContainer(
  { address, passkey }: Server,
  name: string
) {
  return (await axios
    .get(`${address}/container/delete/${name}`, {
      headers: {
        Authorization: passkey || SECRETS.PASSKEY,
      },
    })
    .then(({ data }) => data)) as CommandLogError;
}
