import { Deployment } from "@monitor/types";
import mongoose from "mongoose";
import deploymentModel from "./deployment";
import serverModel from "./server";
import userModel from "./user";

export async function addInitialDocs(mongoURL: string, localMongo: boolean, localRegistry: boolean) {
  await mongoose.connect(mongoURL);

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
    name: "Monitor Core",
    containerName: "monitor-core",
    image: "mbecker2020/monitor-core",
    latest: true,
    serverID: coreServerID,
    owner: "admin",
  };
  deployments.create(coreDeployment);

  if (localMongo) {
     const mongoDeployment: Deployment = {
       name: "Mongo DB",
       containerName: "mongo-db",
       image: "mongo",
       latest: true,
       owner: "admin",
     };
     deployments.create(mongoDeployment);
  }

  if (localRegistry) {
    const registryDeployment: Deployment = {
      name: "Registry",
      containerName: "registry",
      image: "registry:2",
      serverID: coreServerID,
      owner: "admin",
    };
    deployments.create(registryDeployment);
  }
}
