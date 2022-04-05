import { CommandLogError, Deployment, Server } from "@monitor/types";
import { generateQuery } from "@monitor/util";
import axios from "axios";
import { SECRETS } from "../../config";

export async function deployPeriphery(
  { address }: Server,
  deployment: Deployment,
  image?: string,
  dockerAccount?: string
) {
  return (await axios
    .post(
      `${address}/deploy${generateQuery({ image })}`,
      {
        ...deployment,
        dockerAccount: dockerAccount || deployment.dockerAccount,
      },
      {
        headers: {
          Authorization: SECRETS.PASSKEY,
        },
      }
    )
    .then(({ data }) => data)) as CommandLogError;
}
