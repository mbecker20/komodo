import { Collection, Deployment, Server } from "@monitor/types";
import { objFrom2Arrays } from "@monitor/util";
import { FastifyInstance } from "fastify";
import { Model } from "mongoose";

export async function findCollection<T>(
  model: Model<T>,
  filter: object = {}
): Promise<Collection<T>> {
  const docs = await model.find(filter);
  return objFrom2Arrays(
    docs.map((doc) => doc._id),
    docs
  );
}

export async function findByOwner<T>(
  model: Model<T>,
  username: string,
  additionalIDs?: string[]
): Promise<Collection<T>> {
  const docs = await model.find({
    $or: [{ owner: username }, { _id: { $in: additionalIDs } }],
  });
  return objFrom2Arrays(
    docs.map((doc) => doc._id),
    docs
  );
}

export async function findMostRecent<T>(
  model: Model<T>,
  limit: number,
  filter: object = {}
): Promise<T[]> {
  return await model.find(filter).sort({ createdAt: -1 }).limit(limit);
}

export async function getDeploymentAndServer(
  app: FastifyInstance,
  deploymentID: string
) {
  const deployment = (await app.deployments.findById(
    deploymentID
  )) as Deployment;
  return {
    deployment,
    server: (await app.servers.findById(deployment.serverID)) as Server,
  };
}
