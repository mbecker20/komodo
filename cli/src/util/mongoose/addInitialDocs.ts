import { Deployment, Update } from "@monitor/types";
import mongoose from "mongoose";
import { DEFAULT_PORT, DOCKER_NETWORK } from "../../config";
import { Config } from "../../types";
import { timestamp, toDashedName } from "../helpers/general";
import deploymentModel from "./deployment";
import serverModel from "./server";
import updateModel from "./update";
import userModel from "./user";

export async function addInitialDocs({ core, mongo, registry }: Config) {
  await mongoose.connect(
    mongo?.startConfig
      ? mongo!.url.replaceAll(toDashedName(mongo!.startConfig!.name), "127.0.0.1")
      : mongo!.url
  );

  const servers = serverModel();
  const deployments = deploymentModel();
  const updates = updateModel();
  const users = userModel();

  const coreServer = {
    name: "core server",
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
    volumes: [
      { local: core?.secretVolume!, container: "/secrets", useSystemRoot: true },
      { local: "/var/run/docker.sock", container: "/var/run/docker.sock", useSystemRoot: true },
    ],
    ports: [
      { local: core?.port.toString()!, container: DEFAULT_PORT.toString() },
    ],
    environment: [
      { variable: "MONGO_URL", value: mongo!.url },
      { variable: "REGISTRY_URL", value: registry!.url },
    ],
    network: DOCKER_NETWORK,
    serverID: coreServerID,
    owners: ["admin"],
  };
  await deployments.create(coreDeployment);

  if (mongo?.startConfig) {
    const mongoDeployment: Deployment = {
      name: mongo.startConfig.name,
      containerName: toDashedName(mongo.startConfig.name),
      ports: [{ local: mongo.startConfig.port.toString(), container: "27017" }],
      volumes: mongo.startConfig.volume
        ? [{ local: mongo.startConfig.volume, container: "/data/db", useSystemRoot: true }]
        : undefined,
      restart: mongo.startConfig.restart,
      image: "mongo",
      latest: true,
      network: DOCKER_NETWORK,
      owners: ["admin"],
      serverID: coreServerID,
    };
    await deployments.create(mongoDeployment);
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
              useSystemRoot: true,
            },
          ]
        : undefined,
      restart: registry.startConfig.restart,
      image: "registry:2",
      network: DOCKER_NETWORK,
      serverID: coreServerID,
      owners: ["admin"],
    };
    await deployments.create(registryDeployment);
  }

  const startupUpdate: Update = {
    operation: "Startup",
    command: "Start monitor",
    log: {
      stdout: "monitor started successfully",
    },
    timestamp: timestamp(),
    note: "",
    operator: "admin"
  }

  await updates.create(startupUpdate);
}
