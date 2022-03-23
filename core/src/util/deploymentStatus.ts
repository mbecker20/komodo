import { ContainerStatus, Deployment } from "@monitor/types";
import { objFrom2Arrays } from "@monitor/util";
import { FastifyInstance } from "fastify";

export async function deploymentStatusLocal(
  app: FastifyInstance,
  deploymentsAr: Deployment[]
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
    statusNames.map((name) => {
      const tryDeployment = getDeployment(status[name]!, deploymentsAr);
      const _id = tryDeployment ? tryDeployment._id : "none";
      return {
        ...status[name],
        deploymentID: _id,
      };
    })
  );
}

function getDeployment(
  status: ContainerStatus,
  deploymentsAr: Deployment[]
) {
  for (let i = 0; i < deploymentsAr.length; i++) {
    if (deploymentsAr[i].containerName === status.name) {
      return deploymentsAr[i];
    }
  }
  return undefined;
}


