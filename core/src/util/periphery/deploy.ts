import { CommandLogError, Deployment } from "@monitor/types";
import axios from "axios";

export async function deployPeriphery(url: string, passkey: string, deployment: Deployment) {
	return (await axios.post(
    `${url}/deploy`,
    { deployment },
    {
      headers: {
        Authorization: passkey,
      },
    }
  )) as CommandLogError;
}