import { CommandLogError } from "@monitor/types";
import { CORE_IMAGE, PERIPHERY_IMAGE } from "../../config";
import { Config, StartConfig } from "../../types";
import { addInitialDocs } from "../mongoose/mongoose";
import { execute } from "./execute";
import { toDashedName } from "./general";

export type Stage = "mongo" | "registry" | "core" | "periphery" | "docs"

export type Update = { stage: Stage; result?: CommandLogError; description: string };

export default async function deploy(
  config: Config,
  onComplete: (update: Update) => void
) {
  const { core, periphery, mongo, registry } = config;
  if (core) {
    if (mongo && registry) {
      if (mongo.startConfig) {
        const result = await deployMongo(mongo.startConfig);
        onComplete({
          stage: "mongo",
          result,
          description: "",
        });
      }

      if (registry.startConfig) {
        const result = await deployRegistry(registry.startConfig);
        onComplete({
          stage: "registry",
          result,
          description: "",
        })
      }

      const result = await deployCore(config);
      onComplete({
        stage: "core",
        result,
        description: "",
      })

      await addInitialDocs(config);
      onComplete({
        stage: "docs",
        description: ""
      })
    }
  } else if (periphery) {
    const result = await deployPeriphery(config);
    onComplete({
      stage: "periphery",
      result,
      description: "",
    });
  }
}

async function deployCore({ core, mongo, registry }: Config) {
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

async function deployPeriphery({ periphery }: Config) {
  const { name, hostNetwork, port } = periphery!;
  const nameConfig = `--name ${toDashedName(name)}`;
  const network = hostNetwork
    ? `--network="host" -e PORT=${port}`
    : `-p ${port}:9000`;
  const command = `docker run -d ${nameConfig} ${network} ${PERIPHERY_IMAGE}`;
  return await execute(command);
}

async function deployMongo({ name, port, volume, restart }: StartConfig) {
  const command = `docker run -d --name ${name} -p ${port}:27017${
    volume ? ` -v ${volume}:/data/db` : ""
  } --restart ${restart} mongo:latest`;
  return await execute(command);
}

async function deployRegistry({ name, port, volume, restart }: StartConfig) {
  const command = `docker run -d --name ${name} -p ${port}:5000${
    volume ? ` -v ${volume}:/var/lib/registry` : ""
  } --restart ${restart} registry:2`;
  return await execute(command);
}
