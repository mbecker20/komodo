import { CommandLogError, Deployment, Server } from "@monitor/types";
import { generateQuery } from "@monitor/util";
import axios from "axios";
import { SECRETS } from "../../config";

export async function deployPeriphery(
  { address, passkey }: Server,
  deployment: Deployment,
  image?: string,
  dockerAccount?: string
) {
  return (await axios
    .post(
      `${address}/deploy${generateQuery({ image })}`,
      {
        deployment: {
          ...deployment,
          dockerAccount: dockerAccount || deployment.dockerAccount,
        },
      },
      {
        headers: {
          Authorization: passkey || SECRETS.PASSKEY,
        },
      }
    )
    .then(({ data }) => data)) as CommandLogError;
}
