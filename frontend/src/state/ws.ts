import { client, pushNotification, UPDATE_WS_URL } from "..";
import { State } from "./StateProvider";
import { createSignal } from "solid-js";
import ReconnectingWebSocket from "reconnecting-websocket";
import { Operation, Update, UpdateStatus, UpdateTarget } from "../types";

function connectToWs(state: State) {
  const ws = new ReconnectingWebSocket(UPDATE_WS_URL);

  const [isOpen, setOpen] = createSignal(false);

  ws.addEventListener("open", () => {
    // console.log("connection opened");
    ws.send(client.token!);
    setOpen(true);
  });

  ws.addEventListener("message", ({ data }) => {
    if (data === "LOGGED_IN") {
      console.log("logged in to ws");
      return;
    }
    const update = JSON.parse(data) as Update;
    // console.log(message);
    handleMessage(state, update);
  });

  // const int = setInterval(() => {
  //   if (ws.readyState === ws.OPEN) {
  //     ws.send("PING");
  //   } else {
  //     setOpen(false);
  //   }
  // }, 10000);

  ws.addEventListener("close", () => {
    console.log("connection closed");
    // clearInterval(int);
    setOpen(false);
  });

  return {
    subscribe: (
      operations: Operation[],
      callback: (update: Update) => void
    ) => {
      const listener = ({ data }: { data: string }) => {
        if (data === "PONG") return;
        if (data === "LOGGED_IN") return;
        const update = JSON.parse(data) as Update;
        if (operations.length === 0 || operations.includes(update.operation)) {
          callback(update);
        }
      };
      ws.addEventListener("message", listener);
      return () => {
        ws.removeEventListener("message", listener);
      };
    },
    close: () => ws.close(),
    isOpen,
  };
}

async function handleMessage(
  { deployments, builds, servers, groups, procedures, updates }: State,
  update: Update
) {
  updates.addOrUpdate(update);
  let name = "";
  if (update.target.type === "Deployment") {
    const deployment = deployments.get(update.target.id);
    name = deployment ? deployment.deployment.name : "";
  } else if (update.target.type === "Build") {
    const build = builds.get(update.target.id);
    name = build ? build.name : "";
  } else if (update.target.type === "Server") {
    const server = servers.get(update.target.id);
    name = server ? server.server.name : "";
  } else if (update.target.type === "Group") {
    const group = groups.get(update.target.id);
    name = group ? group.name : "";
  } else if (update.target.type === "Procedure") {
    const procedure = procedures.get(update.target.id);
    name = procedure ? procedure.name : "";
  }
  pushNotification(
    update.status === UpdateStatus.InProgress
      ? "ok"
      : update.success
      ? "good"
      : "bad",
    `${
      update.operation === Operation.BuildBuild
        ? "build"
        : update.operation.replaceAll("_", " ")
    } ${name ? `on ${name} ` : ""}(${update.status})`
  );

  // deployment
  if (update.operation === Operation.CreateDeployment) {
    const deployment = await client.get_deployment(update.target.id!);
    deployments.add(deployment);
  } else if (update.operation === Operation.DeleteDeployment) {
    if (update.status === UpdateStatus.Complete) {
      deployments.delete(update.target.id!);
    }
  } else if (update.operation === Operation.UpdateDeployment) {
    if (update.status === UpdateStatus.Complete) {
      const deployment = await client.get_deployment(update.target.id!);
      deployments.update(deployment);
    }
  } else if (
    [
      Operation.DeployContainer,
      Operation.StartContainer,
      Operation.StopContainer,
      Operation.RemoveContainer,
    ].includes(update.operation)
  ) {
    const deployment = await client.get_deployment(update.target.id!);
    deployments.update(deployment);
  }

  // build
  else if (update.operation === Operation.CreateBuild) {
    const build = await client.get_build(update.target.id!);
    builds.add(build);
  } else if (update.operation === Operation.DeleteBuild) {
    if (update.status === UpdateStatus.Complete) {
      builds.delete(update.target.id!);
    }
  } else if (
    [Operation.UpdateBuild, Operation.BuildBuild].includes(update.operation)
  ) {
    if (update.status === UpdateStatus.Complete) {
      const build = await client.get_build(update.target.id!);
      builds.update(build);
    }
  }

  // server
  else if (update.operation === Operation.CreateServer) {
    const server = await client.get_server(update.target.id!);
    servers.add(server);
  } else if (update.operation === Operation.DeleteServer) {
    if (update.status === UpdateStatus.Complete) {
      servers.delete(update.target.id!);
    }
  } else if (update.operation === Operation.UpdateServer) {
    if (update.status === UpdateStatus.Complete) {
      const server = await client.get_server(update.target.id!);
      servers.update(server);
    }
  }

  // group
  else if (update.operation === Operation.CreateGroup) {
    const group = await client.get_group(update.target.id!);
    groups.add(group);
  } else if (update.operation === Operation.DeleteGroup) {
    if (update.status === UpdateStatus.Complete) {
      groups.delete(update.target.id!);
    }
  } else if (update.operation === Operation.UpdateGroup) {
    if (update.status === UpdateStatus.Complete) {
      const group = await client.get_group(update.target.id!);
      groups.update(group);
    }
  }

  // procedure
  else if (update.operation === Operation.CreateProcedure) {
    const procedure = await client.get_procedure(update.target.id!);
    procedures.add(procedure);
  } else if (update.operation === Operation.DeleteProcedure) {
    if (update.status === UpdateStatus.Complete) {
      procedures.delete(update.target.id!);
    }
  } else if (update.operation === Operation.UpdateProcedure) {
    if (update.status === UpdateStatus.Complete) {
      const procedure = await client.get_procedure(update.target.id!);
      procedures.update(procedure);
    }
  }

  // permissions
  else if (update.operation === Operation.ModifyUserPermissions) {
    if (update.status === UpdateStatus.Complete) {
      if (update.target.type === "Build") {
        const build = await client.get_build(update.target.id);
        builds.update(build);
      } else if (update.target.type === "Deployment") {
        const deployment = await client.get_deployment(update.target.id);
        deployments.update(deployment);
      } else if (update.target.type === "Server") {
        const server = await client.get_server(update.target.id);
        servers.update(server);
      } else if (update.target.type === "Group") {
        const group = await client.get_group(update.target.id);
        groups.update(group);
      } else if (update.target.type === "Procedure") {
        const procedure = await client.get_procedure(update.target.id);
        procedures.update(procedure);
      }
    }
  }
}

export default connectToWs;
