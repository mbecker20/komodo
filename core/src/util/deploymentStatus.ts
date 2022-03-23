import { objFrom2Arrays } from "@monitor/util";
import { FastifyInstance } from "fastify";

export async function deploymentStatusLocal(
  app: FastifyInstance,
) {
  const statusAr = await app.dockerode.listContainers({ all: true });
  const statusNames = statusAr.map((stat) =>
    stat.Names[0].slice(1, stat.Names[0].length)
  ); // they all start with '/'
  const status = objFrom2Arrays(
    statusNames,
    statusAr.map((stat, i) => ({
      name: statusNames[i],
      Status: stat.Status,
      State: stat.State as "running" | "exited",
    }))
  );
  return objFrom2Arrays(
    statusNames,
    statusAr.map((stat, i) => ({
      name: statusNames[i],
      Status: stat.Status,
      State: stat.State as "running" | "exited",
    }))
  );
}


