import { CommandLogError, Deployment, Server } from "@monitor/types";
import axios from "axios";
import { SECRETS } from "../../config";

export async function clonePeriphery(
  { address, passkey }: Server,
  deployment: Deployment
) {
  return (await axios
    .post(
      `${address}/repo/clone`,
      { deployment },
      {
        headers: {
          Authorization: passkey || SECRETS.PASSKEY,
        },
      }
    )
    .then(({ data }) => data)) as CommandLogError;
}

export async function pullPeriphery(
  { address, passkey }: Server,
  deployment: Deployment
) {
  return (await axios
    .post(
      `${address}/repo/pull`,
      { deployment },
      {
        headers: {
          Authorization: passkey || SECRETS.PASSKEY,
        },
      }
    )
    .then(({ data }) => data)) as CommandLogError;
}

export async function deleteRepoPeriphery(
  { address, passkey }: Server,
  deployment: Deployment
) {
  return (await axios
    .post(
      `${address}/repo/delete`,
      { deployment },
      {
        headers: {
          Authorization: passkey || SECRETS.PASSKEY,
        },
      }
    )
    .then(({ data }) => data)) as CommandLogError;
}
