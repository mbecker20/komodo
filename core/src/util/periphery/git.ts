import { CommandLogError, Deployment } from "@monitor/types";
import axios from "axios";

export async function clonePeriphery(
  url: string,
  passkey: string,
  deployment: Deployment
) {
  return (await axios.post(
    `${url}/repo/clone`,
    { deployment },
    {
      headers: {
        Authorization: passkey,
      },
    }
  )) as CommandLogError;
}

export async function pullPeriphery(
  url: string,
  passkey: string,
  deployment: Deployment
) {
  return (await axios.post(
    `${url}/repo/pull`,
    { deployment },
    {
      headers: {
        Authorization: passkey,
      },
    }
  )) as CommandLogError;
}

export async function deleteRepoPeriphery(
  url: string,
  passkey: string,
  deployment: Deployment
) {
  return (await axios.post(
    `${url}/repo/delete`,
    { deployment },
    {
      headers: {
        Authorization: passkey,
      },
    }
  )) as CommandLogError;
}
