import { StartConfig } from "../../types";
import { execute } from "../execute";

export async function startMongo({ name, port, volume, restart }: StartConfig) {
  const command = `docker run -d --name ${name} -p ${port}:27017${
    volume ? ` -v ${volume}:/data/db` : ""
  } --restart ${restart} mongo:latest`;
  return await execute(command);
}
