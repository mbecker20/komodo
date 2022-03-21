import { Deployment } from "@monitor/types";
import mongoose from "mongoose";
import { DEFAULT_PORT } from "../../config";
import { Config } from "../../types";
import { toDashedName } from "../helpers/general";
import deploymentModel from "./deployment";
import serverModel from "./server";
import userModel from "./user";

export async function addInitialDocs({ core, mongo, registry }: Config) {
  await mongoose.connect(mongo!.url);

  const servers = serverModel();
  const deployments = deploymentModel();
  const users = userModel();

  const coreServer = {
    name: "Core Server",
    address: "localhost",
    passkey: "passkey",
    enabled: true,
    isCore: true,
  };
  const coreServerID = (await servers.create(coreServer)).toObject()._id;

  const coreDeployment: Deployment = {
    name: core!.name,
    containerName: toDashedName(core!.name),
    image: "mbecker2020/monitor-core",
    latest: true,
    restart: core?.restart,
    volumes: [{ local: core?.secretVolume!, container: "/secrets" }],
    ports: core?.hostNetwork
      ? undefined
      : [{ local: core?.port.toString()!, container: DEFAULT_PORT.toString() }],
    environment: [
      { variable: "MONGO_URL", value: mongo!.url },
      { variable: "REGISTRY_URL", value: registry!.url },
      (core?.hostNetwork
        ? { variable: "PORT", value: core.port.toString() }
        : undefined)!,
    ].filter((val) => val),
    serverID: coreServerID,
    owner: "admin",
  };
  deployments.create(coreDeployment);

  if (mongo?.startConfig) {
    const mongoDeployment: Deployment = {
      name: mongo.startConfig.name,
      containerName: toDashedName(mongo.startConfig.name),
      ports: [{ local: mongo.startConfig.port.toString(), container: "27017" }],
      volumes: mongo.startConfig.volume
        ? [{ local: mongo.startConfig.volume, container: "/data/db" }]
        : undefined,
      restart: mongo.startConfig.restart,
      image: "mongo",
      latest: true,
      owner: "admin",
      serverID: coreServerID,
    };
    deployments.create(mongoDeployment);
  }

  if (registry?.startConfig) {
    const registryDeployment: Deployment = {
      name: registry.startConfig.name,
      containerName: toDashedName(registry.startConfig.name),
      ports: [
        { local: registry.startConfig.port.toString(), container: "5000" },
      ],
      volumes: registry.startConfig.volume
        ? [
            {
              local: registry.startConfig.volume,
              container: "/var/lib/registry",
            },
          ]
        : undefined,
      restart: registry.startConfig.restart,
      image: "registry:2",
      serverID: coreServerID,
      owner: "admin",
    };
    deployments.create(registryDeployment);
  }
}
