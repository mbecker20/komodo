import { CommandLogError } from "@monitor/types";
import {
  CORE_IMAGE,
  DEFAULT_PERIPHERY_PORT,
  DEFAULT_PORT,
  DOCKER_NETWORK,
  PERIPHERY_IMAGE,
} from "../../config";
import { Config, StartConfig } from "../../types";
import { addInitialDocs } from "../mongoose/addInitialDocs";
import { deleteContainer } from "./docker";
import { execute } from "./execute";
import { noTrailingSlash, toDashedName, trailingSlash } from "./general";

export type Stage = "mongo" | "registry" | "core" | "periphery" | "docs";

export type Update = {
  stage: Stage;
  result?: CommandLogError;
  description: string;
};

export default async function deploy(
  config: Config,
  onComplete: (update: Update) => void
) {
  const { core, periphery, mongo } = config;
  if (core) {
    if (mongo) {
      await createNetwork();

      if (mongo.startConfig) {
        const result = await deployMongo(mongo.startConfig);
        onComplete({
          stage: "mongo",
          result,
          description: "mongo started",
        });
      }

      // if (registry.startConfig) {
      //   const result = await deployRegistry(registry.startConfig);
      //   onComplete({
      //     stage: "registry",
      //     result,
      //     description: "registry started",
      //   });
      // }

      const result = await deployCore(config);
      onComplete({
        stage: "core",
        result,
        description: "monitor core started",
      });

      await addInitialDocs(config);
      onComplete({
        stage: "docs",
        description: "configurations added to db",
      });
    }
  } else if (periphery) {
    await deleteContainer(toDashedName(periphery.name));
    const result = await deployPeriphery(config);
    onComplete({
      stage: "periphery",
      result,
      description: "monitor periphery started",
    });
  }
}

async function deployCore({ core, mongo }: Config) {
  await execute("docker pull mbecker2020/monitor-core:latest");
  const { name, secretVolume, port, restart, sysroot, host } = core!;
  const nameConfig = `--name ${toDashedName(name)}`;
  const volumes = `-v ${secretVolume}:/secrets -v /var/run/docker.sock:/var/run/docker.sock -v ${sysroot}:/monitor-root`;
  const network = `-p ${port}:${DEFAULT_PORT} --network ${DOCKER_NETWORK}`;
  const env = `-e MONGO_URL=${mongo?.url} -e SYSROOT=${trailingSlash(
    core?.sysroot!
  )} -e HOST=${noTrailingSlash(host!)}`;
  const restartArg = `--restart ${restart}`;
  const command = `docker run -d ${nameConfig} ${volumes} ${network} ${env} ${restartArg} ${CORE_IMAGE}`;
  return await execute(command);
}

async function deployPeriphery({ periphery }: Config) {
  await execute("docker pull mbecker2020/monitor-periphery:latest");
  const { name, port, secretVolume, restart, sysroot } = periphery!;
  const nameConfig = `--name ${toDashedName(name)}`;
  const volume = `-v ${secretVolume}:/secrets -v /var/run/docker.sock:/var/run/docker.sock -v ${sysroot}:/monitor-root`;
  const network = `-p ${port}:${DEFAULT_PERIPHERY_PORT}`;
  const env = `-e SYSROOT=${trailingSlash(periphery?.sysroot!)}`;
  const restartArg = `--restart ${restart}`;
  const command = `docker run -d ${nameConfig} ${volume} ${network} ${env} ${restartArg} ${PERIPHERY_IMAGE}`;
  return await execute(command);
}

async function deployMongo({ name, port, volume, restart }: StartConfig) {
  const command = `docker run -d --name ${name} -p ${port}:27017${
    volume ? ` -v ${volume}:/data/db` : ""
  } --network ${DOCKER_NETWORK} --restart ${restart} mongo:latest`;
  return await execute(command);
}

// async function deployRegistry({ name, port, volume, restart }: StartConfig) {
//   const command = `docker run -d --name ${name} -p ${port}:5000${
//     volume ? ` -v ${volume}:/var/lib/registry` : ""
//   } --network ${DOCKER_NETWORK} --restart ${restart} registry:2`;
//   return await execute(command);
// }

async function createNetwork() {
  const command = `docker network create ${DOCKER_NETWORK}`;
  return await execute(command);
}
