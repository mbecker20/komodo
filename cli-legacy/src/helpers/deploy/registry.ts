import { StartConfig } from "../../types";
import { execute } from "../execute";

export async function startRegistry({
  name,
  port,
  volume,
  restart,
}: StartConfig) {
  const command = `docker run -d --name ${name} -p ${port}:5000${
    volume ? ` -v ${volume}:/var/lib/registry` : ""
  } --restart ${restart} registry:2`;
  return await execute(command);
}
