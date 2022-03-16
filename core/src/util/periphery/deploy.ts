import { CommandLogError, Deployment, Server } from "@monitor/types";
import axios from "axios";

export async function deployPeriphery(
  { address, passkey }: Server,
  deployment: Deployment
) {
  return (await axios.post(
    `http://${address}/deploy`,
    { deployment },
    {
      headers: {
        Authorization: passkey,
      },
    }
  )) as CommandLogError;
}