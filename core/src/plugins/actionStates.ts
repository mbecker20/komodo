import {
  BuildActionState,
  BuildActionStates,
  DeployActionState,
  DeployActionStates,
} from "@monitor/types";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";

interface ActionState {
  getJSON(id: string): void;
  add(id: string): void;
  delete(id: string): void;
  set(
    id: string,
    type: keyof BuildActionState | keyof DeployActionState,
    state: boolean
  ): void;
  get(
    id: string,
    type: keyof BuildActionState | keyof DeployActionState
  ): boolean;
  getMultiple(
    id: string,
    types: (keyof BuildActionState | keyof DeployActionState)[]
  ): boolean;
}

declare module "fastify" {
  interface FastifyInstance {
    buildActionStates: ActionState;
    deployActionStates: ActionState;
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
  });

  app.builds.find({}, { _id: true }).then((builds) => {
    builds.forEach((build) => app.buildActionStates.add(build._id!));
  });
  app.deployments.find({}, { _id: true }).then((deployments) => {
    deployments.forEach((deployment) =>
      app.deployActionStates.add(deployment._id!)
    );
  });

  done();
});

export default actionStates;
