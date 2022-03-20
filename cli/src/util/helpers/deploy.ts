import { CORE_IMAGE } from "../../config";
import { Config, CoreConfig, StartConfig } from "../../types";
import { execute } from "./execute";
import { toDashedName } from "./general";

export async function deployCore({ core, mongo, registry }: Config) {
  const { name, hostNetwork, secretVolume, port } = core!;
  const nameConfig = `--name ${toDashedName(name)}`;
  const volume = `-v ${secretVolume}:/secrets`;
  const network = hostNetwork ? '--network="host"' : `-p ${port}:9000`;
  const env = `-e MONGO_URL=${mongo?.url} -e REGISTRY_URL=${registry?.url}${
    hostNetwork ? ` -e PORT=${port}` : ""
  }`;
  const command = `docker run -d ${nameConfig} ${volume} ${network} ${env} ${CORE_IMAGE}`;
  return await execute(command);
}

export async function deployMongo({
  name,
  port,
  volume,
  restart,
}: StartConfig) {
  const command = `docker run -d --name ${name} -p ${port}:27017${
    volume ? ` -v ${volume}:/data/db` : ""
  } --restart ${restart} mongo:latest`;
  return await execute(command);
}

export async function deployRegistry({
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
