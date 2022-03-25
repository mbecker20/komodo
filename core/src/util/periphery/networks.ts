import { Network, Server } from "@monitor/types";
import axios from "axios";

export async function getPeripheryNetworks({ address, passkey }: Server) {
  return await axios
    .get<Network[]>(`http://${address}/networks`, {
      headers: {
        Authorization: passkey,
      },
    })
    .then(({ data }) => data);
}
