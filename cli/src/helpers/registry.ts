import { execute } from "./execute";

export async function startRegistry(
  name: string,
  port: number,
  volume: string | false,
  restart: string
) {
	const command = `docker run -d --name ${name} -p ${port}:5000${
    volume ? ` -v ${volume}:/var/lib/registry` : ""
  } --restart ${restart} registry:2`;
  return await execute(command);
}