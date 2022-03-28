import { CommandLogError, Deployment, Server } from "@monitor/types";
import axios from "axios";

export async function clonePeriphery(
  { address, passkey }: Server,
  deployment: Deployment
) {
  return (await axios.post(
    `${address}/repo/clone`,
    { deployment },
    {
      headers: {
        Authorization: passkey,
      },
    }
  )) as CommandLogError;
}

export async function pullPeriphery(
  { address, passkey }: Server,
  deployment: Deployment
) {
  return (await axios.post(
    `${address}/repo/pull`,
    { deployment },
    {
      headers: {
        Authorization: passkey,
      },
    }
  )) as CommandLogError;
}

export async function deleteRepoPeriphery(
  { address, passkey }: Server,
  deployment: Deployment
) {
  return (await axios.post(
    `${address}/repo/delete`,
    { deployment },
    {
      headers: {
        Authorization: passkey,
      },
    }
  )) as CommandLogError;
}
