import { Collection, CommandLogError, ContainerStatus, Log } from "@monitor/types";
import axios from "axios";

export async function getPeripheryContainers(url: string, passkey: string) {
  return (await axios.get(`${url}/containers`, {
    headers: {
      Authorization: passkey,
    },
  })) as Collection<ContainerStatus>;
}

export async function getPeripheryContainer(
  url: string,
  passkey: string,
  name: string
) {
	return (await axios.get(`${url}/container/${name}`, {
    headers: {
      Authorization: passkey,
    },
  })) as ContainerStatus | "not created";
}

export async function getPeripheryContainerLog(
  url: string,
  passkey: string,
  name: string
) {
  return (await axios.get(`${url}/container/log/${name}`, {
    headers: {
      Authorization: passkey,
    },
  })) as Log;
}

export async function startPeripheryContainer(
  url: string,
  passkey: string,
  name: string
) {
	return (await axios.get(`${url}/container/start/${name}`, {
    headers: {
      Authorization: passkey,
    },
  })) as CommandLogError;
}

export async function stopPeripheryContainer(
  url: string,
  passkey: string,
  name: string
) {
  return (await axios.get(`${url}/container/stop/${name}`, {
    headers: {
      Authorization: passkey,
    },
  })) as CommandLogError;
}

export async function deletePeripheryContainer(
  url: string,
  passkey: string,
  name: string
) {
  return (await axios.get(`${url}/container/delete/${name}`, {
    headers: {
      Authorization: passkey,
    },
  })) as CommandLogError;
}


