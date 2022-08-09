import {
  BuildActionState,
  BuildActionStates,
  DeployActionState,
  DeployActionStates,
  ServerActionState,
  ServerActionStates,
} from "@monitor/types";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";

interface ActionState<T> {
  getJSON(id: string): T;
  add(id: string): void;
  delete(id: string): void;
  set(id: string, type: keyof T, state: boolean): void;
  get(id: string, type: keyof T): boolean;
  getMultiple(id: string, types: (keyof T)[]): boolean;
  busy(id: string): boolean;
}

declare module "fastify" {
  interface FastifyInstance {
    buildActionStates: ActionState<BuildActionState>;
    deployActionStates: ActionState<DeployActionState>;
    serverActionStates: ActionState<ServerActionState>;
  }
}

export const PULLING = "pulling";
export const BUILDING = "building";
export const CLONING = "cloning";
export const DEPLOYING = "deploying";
export const STARTING = "starting";
export const STOPPING = "stopping";
export const DELETING = "deleting";

const actionStates = fp((app: FastifyInstance, _: {}, done: () => void) => {
  const buildActionStates: BuildActionStates = {};
  const deployActionStates: DeployActionStates = {};
  const serverActionStates: ServerActionStates = {};

  app.decorate("buildActionStates", {
    getJSON: (buildID: string) => {
      return buildActionStates[buildID];
    },
    add: (buildID: string) => {
      buildActionStates[buildID] = {
        pulling: false,
        building: false,
        cloning: false,
        updating: false,
        deleting: false,
      };
    },
    delete: (buildID: string) => {
      delete buildActionStates[buildID];
    },
    set: (buildID: string, type: string, state: boolean) => {
      buildActionStates[buildID][type] = state;
    },
    get: (buildID: string, type: string) => {
      return buildActionStates[buildID][type];
    },
    getMultiple: (buildID: string, types: string[]) => {
      for (const type of types) {
        if (buildActionStates[buildID][type]) return true;
      }
      return false;
    },
    busy: (buildID: string) => {
      for (const type of [
        "pulling",
        "building",
        "cloning",
        "updating",
        "deleting",
      ]) {
        if (buildActionStates[buildID][type]) return true;
      }
      return false;
    },
  });

  app.decorate("deployActionStates", {
    getJSON: (deploymentID: string) => {
      return deployActionStates[deploymentID];
    },
    add: (deploymentID: string) => {
      deployActionStates[deploymentID] = {
        deploying: false,
        deleting: false,
        starting: false,
        stopping: false,
        updating: false,
        fullDeleting: false,
        pulling: false,
        recloning: false,
      };
    },
    delete: (deploymentID: string) => {
      delete deployActionStates[deploymentID];
    },
    set: (deploymentID: string, type: string, state: boolean) => {
      deployActionStates[deploymentID][type] = state;
    },
    get: (deploymentID: string, type: string) => {
      return deployActionStates[deploymentID][type];
    },
    getMultiple: (deploymentID: string, types: string[]) => {
      for (const type of types) {
        if (deployActionStates[deploymentID][type]) return true;
      }
      return false;
    },
    busy: (deploymentID: string) => {
      for (const type of [
        "deploying",
        "deleting",
        "starting",
        "stopping",
        "updating",
        "fullDeleting",
        "pulling",
        "recloning"
      ]) {
        if (deployActionStates[deploymentID][type]) return true;
      }
      return false;
    },
  });

  app.decorate("serverActionStates", {
    getJSON: (serverID: string) => {
      return serverActionStates[serverID];
    },
    add: (serverID: string) => {
      serverActionStates[serverID] = {
        pruningImages: false,
        pruningNetworks: false,
        pruningContainers: false,
        deleting: false,
      };
    },
    delete: (serverID: string) => {
      delete serverActionStates[serverID];
    },
    set: (serverID: string, type: string, state: boolean) => {
      serverActionStates[serverID][type] = state;
    },
    get: (serverID: string, type: string) => {
      return serverActionStates[serverID][type];
    },
    getMultiple: (serverID: string, types: string[]) => {
      for (const type of types) {
        if (serverActionStates[serverID][type]) return true;
      }
      return false;
    },
    busy: (serverID: string) => {
      for (const type of ["pruningImages", "pruningNetworks", "pruningContainers", "deleting"]) {
        if (serverActionStates[serverID][type]) return true;
      }
      return false;
    },
  });

  app.builds.find({}, { _id: true }).then((builds) => {
    builds.forEach((build) => app.buildActionStates.add(build._id!));
  });
  app.deployments.find({}, { _id: true }).then((deployments) => {
    deployments.forEach((deployment) =>
      app.deployActionStates.add(deployment._id!)
    );
  });
  app.servers.find({}, { _id: true }).then((servers) => {
    servers.forEach((server) => app.serverActionStates.add(server._id!));
  });

  done();
});

export default actionStates;
