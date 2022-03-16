import { Collection, CommandLogError, ContainerStatus, Log, Server } from "@monitor/types";
import axios from "axios";

export async function getPeripheryContainers({ address, passkey }: Server) {
  return (await axios.get(`http://${address}/containers`, {
    headers: {
      Authorization: passkey,
    },
  })) as Collection<ContainerStatus>;
}

export async function getPeripheryContainer(
  { address, passkey }: Server,
  name: string
) {
	return (await axios.get(`http://${address}/container/${name}`, {
    headers: {
      Authorization: passkey,
    },
  })) as ContainerStatus | "not created";
}

export async function getPeripheryContainerLog(
  { address, passkey }: Server,
  name: string
) {
  return (await axios.get(`http://${address}/container/log/${name}`, {
    headers: {
      Authorization: passkey,
    },
  })) as Log;
}

export async function startPeripheryContainer(
  { address, passkey }: Server,
  name: string
) {
  return (await axios.get(`http://${address}/container/start/${name}`, {
    headers: {
      Authorization: passkey,
    },
  })) as CommandLogError;
}

export async function stopPeripheryContainer(
  { address, passkey }: Server,
  name: string
) {
  return (await axios.get(`http://${address}/container/stop/${name}`, {
    headers: {
      Authorization: passkey,
    },
  })) as CommandLogError;
}

export async function deletePeripheryContainer(
  { address, passkey }: Server,
  name: string
) {
  return (await axios.get(`http://${address}/container/delete/${name}`, {
    headers: {
      Authorization: passkey,
    },
  })) as CommandLogError;
}


