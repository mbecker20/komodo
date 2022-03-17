import { CommandLogError, Deployment, Server } from "@monitor/types";
import axios from "axios";
import { REGISTRY_URL } from "../../config";

export async function deployPeriphery(
  { address, passkey }: Server,
  deployment: Deployment,
  image?: string
) {
  return (await axios.post(
    `http://${address}/deploy${image && "?image=" + REGISTRY_URL + image}`,
    { deployment },
    {
      headers: {
        Authorization: passkey,
      },
    }
  )) as CommandLogError;
}
