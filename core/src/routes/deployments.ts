import { Deployment, Server, User } from "@monitor/types";
import { intoCollection, DEPLOYMENT_OWNER_UPDATE, CREATE_DEPLOYMENT, DEPLOY, prettyStringify } from "@monitor/util";
import { deleteContainer, dockerRun, getContainerLog, getContainerStatus } from "@monitor/util-node";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { PERMISSIONS_DENY_LOG, SECRETS, SYSROOT, SYS_DEPLOYMENT_REPO_PATH } from "../config";
import { DEPLOYING } from "../plugins/actionStates";
import { deploymentStatusLocal } from "../util/deploymentStatus";
import { toDashedName } from "../util/helpers";
import {
  deletePeripheryContainer,
  getPeripheryContainer,
  getPeripheryContainerLog,
  getPeripheryContainers,
} from "../util/periphery/container";
import { serverStatusPeriphery } from "../util/periphery/status";
import { addDeploymentUpdate, addSystemUpdate } from "../util/updates";
import { join } from "path";
import { deployPeriphery } from "../util/periphery/deploy";

async function getDeployments(
  app: FastifyInstance,
  server: Server,
  user: User
) {
  const deployments = await app.deployments.find(
    user.permissions! > 1
      ? { serverID: server._id }
      : { serverID: server._id, owners: user.username },
    "name containerName serverID owners repo isCore"
  );
  if (await serverStatusPeriphery(server)) {
    const status = server.isCore
      ? await deploymentStatusLocal(app)
      : await getPeripheryContainers(server);
    return intoCollection(
      deployments.map((deployment) => {
        deployment.status = status[deployment.containerName!] || "not deployed";
        return deployment;
      })
    );
  } else {
    return intoCollection(
      deployments.map((deployment) => {
        deployment.status = "unknown";
        return deployment;
      })
    );
  }
}

const deployments = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.get(
    "/api/deployments",
    { onRequest: [app.auth, app.userEnabled] },
    async (req, res) => {
      // returns all the deployments
      const user = (await app.users.findById(req.user.id))!;
      const servers = await app.servers.find(
        user.permissions! > 1 ? {} : { owners: user.username }
      );
      const deployments = (
        await Promise.all(
          servers.map((server) => getDeployments(app, server, user))
        )
      ).reduce((acc, curr) => {
        Object.keys(curr).forEach((id) => {
          acc[id] = curr[id];
        });
        return acc;
      }, {});
      res.send(deployments);
    }
  );

  app.get(
    "/api/deployment/:id",
    { onRequest: [app.auth, app.userEnabled] },
    async (req, res) => {
      const { id } = req.params as { id: string };
      const deployment = await app.deployments.findById(id);
      if (!deployment) {
        res.status(400);
        res.send("could not find deployment");
        return;
      }
      const user = await app.users.findById(
        req.user.id,
        "username permissions"
      );
      if (
        !user ||
        (user?.permissions! < 2 && !deployment.owners.includes(user.username))
      ) {
        res.status(403);
        res.send("user does not have permissions to view this information");
        return;
      }
      const onCore = deployment.serverID === app.core._id;
      const server = onCore
        ? app.core
        : await app.servers.findById(deployment.serverID!);
      if (!server) {
        res.status(400);
        res.send("could not find deployment's server");
        return;
      }
      deployment.status = onCore
        ? await getContainerStatus(app.dockerode, deployment.containerName!)
        : await getPeripheryContainer(server, deployment.containerName!);
      res.send(deployment);
    }
  );

  app.get(
    "/api/deployment/:id/log",
    { onRequest: [app.auth, app.userEnabled] },
    async (req, res) => {
      const { id } = req.params as { id: string };
      const { tail } = req.query as { tail?: number };
      const deployment = await app.deployments.findById(
        id,
        "serverID containerName owners"
      );
      if (!deployment) {
        res.status(400);
        res.send("could not find deployment");
        return;
      }
      const user = await app.users.findById(
        req.user.id,
        "username permissions"
      );
      if (
        !user ||
        (user?.permissions! < 2 && !deployment.owners.includes(user.username))
      ) {
        res.status(403);
        res.send("user does not have permissions to view this log");
        return;
      }
      const onCore = deployment.serverID === app.core._id;
      const server = onCore
        ? app.core
        : await app.servers.findById(deployment.serverID!);
      if (!server) {
        res.status(400);
        res.send("could not find deployment's server");
        return;
      }
      const log = onCore
        ? await getContainerLog(deployment.containerName!, tail || 50)
        : await getPeripheryContainerLog(
          server,
          deployment.containerName!,
          tail || 50
        );
      res.send(log);
    }
  );

  app.get(
    "/api/deployment/:id/log/download",
    { onRequest: [app.auth, app.userEnabled] },
    async (req, res) => {
      const { id } = req.params as { id: string };
      const deployment = await app.deployments.findById(
        id,
        "name containerName owners serverID"
      );
      if (!deployment) {
        res.status(400);
        res.send("deployment not found");
        return;
      }
      const user = await app.users.findById(
        req.user.id,
        "username permissions"
      );
      if (
        !user ||
        (user?.permissions! < 2 && !deployment.owners.includes(user.username))
      ) {
        res.status(403);
        res.send("user not authorized for this action");
        return;
      }
      const server = await app.servers.findById(deployment.serverID!);
      if (!server) {
        res.status(400);
        res.send("could not find deployment's server");
        return;
      }
      const log = server.isCore
        ? await getContainerLog(deployment.containerName!)
        : await getPeripheryContainerLog(server, deployment.containerName!);
      res.send(log);
    }
  );

  app.get(
    "/api/deployment/:id/status",
    { onRequest: [app.auth, app.userEnabled] },
    async (req, res) => {
      const { id } = req.params as { id: string };
      const deployment = await app.deployments.findById(
        id,
        "serverID containerName"
      );
      if (!deployment) {
        res.status(400);
        res.send("could not find deployment");
        return;
      }
      const onCore = deployment.serverID === app.core._id;
      const server = onCore
        ? app.core
        : await app.servers.findById(deployment.serverID!);
      if (!server) {
        res.status(400);
        res.send("could not find deployment's server");
        return;
      }
      const status = onCore
        ? await getContainerStatus(app.dockerode, deployment.containerName!)
        : await getPeripheryContainer(server, deployment.containerName!);
      res.send(status);
    }
  );

  app.get(
    "/api/deployment/:id/action-state",
    { onRequest: [app.auth, app.userEnabled] },
    async (req, res) => {
      const { id } = req.params as { id: string };
      const state = app.deployActionStates.getJSON(id);
      res.send(state);
    }
  );

  app.post(
    "/api/deployment/create",
    { onRequest: [app.auth, app.userEnabled] },
    async (req, res) => {
      const { deployment } = req.body as { deployment: Deployment };
      const user = await app.users.findById(req.user.id);
      if (!user) {
        res.status(403);
        res.send("user not found");
        return;
      }
      const server = await app.servers.findById(deployment.serverID!);
      if (!server) {
        return;
      }
      if (
        user.permissions! < 1 ||
        (user.permissions! < 2 && !server.owners.includes(user.username))
      ) {
        addSystemUpdate(
          app,
          CREATE_DEPLOYMENT,
          "Create Deployment (DENIED)",
          PERMISSIONS_DENY_LOG,
          user.username,
          "",
          true
        );
        res.status(403);
        res.send("user does not have permissions")
        return;
      }
      const created = await app.deployments.create({
        ...deployment,
        containerName: toDashedName(deployment.name),
        owners: [user.username],
      });
      app.deployActionStates.add(created._id!);
      addDeploymentUpdate(
        app,
        created._id!,
        CREATE_DEPLOYMENT,
        "Create Deployment",
        { stdout: "Deployment Created: " + deployment.name },
        user.username
      );
      res.send(created);
    }
  );

  app.get("/api/deployment/:deploymentID/deploy", { onRequest: [app.auth, app.userEnabled] }, async (req, res) => {
    const { deploymentID } = req.params as { deploymentID: string };
    const user = await app.users.findById(req.user.id);
    if (!user) {
      res.status(403);
      res.send("user not found");
      return;
    }
    if (app.deployActionStates.busy(deploymentID)) {
      res.status(400);
      res.send("busy, wait a bit");
      return;
    }
    const deployment = await app.deployments.findById(deploymentID);
    if (!deployment) {
      res.status(400);
      res.send("deployment not found")
      return;
    };
    if (user.permissions! < 2 && !deployment.owners.includes(user.username)) {
      addDeploymentUpdate(
        app,
        deploymentID,
        DEPLOY,
        "Deploy (DENIED)",
        PERMISSIONS_DENY_LOG,
        user.username,
        undefined,
        true
      );
      return;
    }
    if (app.deployActionStates.get(deploymentID, DEPLOYING)) return;
    app.deployActionStates.set(deploymentID, DEPLOYING, true);
    app.broadcast(DEPLOY, { complete: false, deploymentID });
    try {
      const server =
        deployment.serverID === app.core._id
          ? undefined
          : await app.servers.findById(deployment.serverID!);
      if (server) {
        // delete the container on periphery
        await deletePeripheryContainer(server, deployment.containerName!);
      } else {
        // delete the container on core
        await deleteContainer(deployment.containerName!);
      }
      const build = deployment.buildID
        ? await app.builds.findById(deployment.buildID)
        : undefined;
      const image =
        build && build.dockerBuildArgs
          ? join(build.dockerAccount || "", build.pullName!)
          : undefined;
      const { command, log, isError } = server
        ? await deployPeriphery(
            server,
            deployment,
            image,
            build && build.dockerAccount
          )
        : await dockerRun(
            {
              ...deployment,
              image: image ? image : deployment.image,
            },
            SYSROOT,
            deployment.repo && deployment.containerMount
              ? {
                  repoFolder: join(
                    SYS_DEPLOYMENT_REPO_PATH,
                    deployment.containerName!,
                    deployment.repoMount || ""
                  ),
                  containerMount: deployment.containerMount,
                }
              : undefined,
            (build && build.dockerAccount) || deployment.dockerAccount,
            ((build && build.dockerAccount) || deployment.dockerAccount) &&
              SECRETS.DOCKER_ACCOUNTS[
                ((build && build.dockerAccount) || deployment.dockerAccount)!
              ]
          );
      addDeploymentUpdate(
        app,
        deploymentID,
        DEPLOY,
        command,
        log,
        user.username,
        undefined,
        isError
      );
    } catch (error) {
      addDeploymentUpdate(
        app,
        deploymentID,
        DEPLOY,
        "Deploy (ERROR)",
        { stderr: prettyStringify(error) },
        user.username,
        undefined,
        true
      );
      res.status(400);
      res.send("deploy failed");
      return;
    }
    app.broadcast(DEPLOY, { complete: true, deploymentID });
    app.deployActionStates.set(deploymentID, DEPLOYING, false);
    res.send("deploy success");
  });

  app.post(
    "/api/deployment/:id/:owner",
    { onRequest: [app.auth, app.userEnabled] },
    async (req, res) => {
      // adds an owner to a deployment
      const { id, owner } = req.params as { id: string; owner: string };
      const sender = (await app.users.findById(req.user.id))!;
      if (sender.permissions! < 1) {
        res.status(403);
        res.send("inadequate permissions");
        return;
      }
      const user = await app.users.findOne({ username: owner });
      if (!user || user.permissions! < 1) {
        res.status(400);
        res.send("invalid user");
        return;
      }
      const deployment = await app.deployments.findById(id);
      if (!deployment) {
        res.status(400);
        res.send("deployment not found");
        return;
      }
      if (
        sender.permissions! < 2 &&
        !deployment.owners.includes(sender.username)
      ) {
        res.status(403);
        res.send("inadequate permissions");
        return;
      }
      await app.deployments.updateById(id, { $push: { owners: owner } });
      app.broadcast(DEPLOYMENT_OWNER_UPDATE, { deploymentID: id });
      res.send("owner added");
    }
  );

  app.delete(
    "/api/deployment/:id/:owner",
    { onRequest: [app.auth, app.userEnabled] },
    async (req, res) => {
      // removes owner from deployment
      const { id, owner } = req.params as { id: string; owner: string };
      const sender = (await app.users.findById(req.user.id))!;
      if (sender.permissions! < 2) {
        res.status(403);
        res.send("inadequate permissions");
        return;
      }
      const deployment = await app.deployments.findById(id);
      if (!deployment) {
        res.status(400);
        res.send("deployment not found");
        return;
      }
      await app.deployments.updateById(id, { $pull: { owners: owner } });
      app.broadcast(DEPLOYMENT_OWNER_UPDATE, { deploymentID: id });
      res.send("owner removed");
    }
  );

  

  done();
});

export default deployments;
