import { CORE_IMAGE } from "../../config";
import { Config } from "../../types";
import { execute } from "../execute";
import { toDashedName } from "../general";

export async function startMonitorCore({
  monitorCore: { name, secretVolume, hostNetwork, port },
  mongo: { url: mongoURL },
  registry: { url: registryURL },
}: Config) {
  const nameConfig = `--name ${toDashedName(name)}`;
  const env = `-e MONGO_URL=${mongoURL}${
    registryURL ? ` -e REGISTRY_URL=${registryURL}` : ""
  }`;
  const volume = `-v ${secretVolume}:/secrets`;
  const network = hostNetwork ? `--network=\"host\"${port ? ` -e PORT=${port}` : ""}` : `-p ${port}:9000`;
  const command = `docker run -d ${nameConfig} ${volume} ${network} ${env} ${CORE_IMAGE}`;
  return await execute(command);
}
