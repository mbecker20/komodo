import {
  ContainerStatus,
  Conversion,
  DockerBuildArgs,
  DockerRunArgs,
  EnvironmentVar,
  Network,
  CommandLogError,
} from "@monitor/types";
import { join } from "path";
import { execute } from "./execute";
import { objFrom2Arrays } from "./helpers";
import Dockerode from "dockerode";

/* Server */

export async function pruneImages() {
  return await execute("docker image prune -a -f");
}

export async function getNetworks(dockerode: Dockerode): Promise<Network[]> {
  const networks = await dockerode.listNetworks();
  return networks.map(({ Name, Driver }) => ({
    // _id: Id,
    name: Name,
    driver: Driver,
  }));
}

export async function createNetwork(
  name: string,
  driver?: string
): Promise<CommandLogError> {
  return await execute(
    `docker network create${driver ? ` -d ${driver}` : ""} ${name}`
  );
}

export async function deleteNetwork(name: string): Promise<CommandLogError> {
  return await execute(`docker network rm ${name}`);
}

export async function pruneNetworks(): Promise<CommandLogError> {
  return await execute(`docker network prune -f`);
}

/* Container */

export async function allContainerStatus(dockerode: Dockerode) {
  const statusAr = await dockerode.listContainers({ all: true });
  const statusNames = statusAr.map(
    (stat) => stat.Names[0]?.slice(1, stat.Names[0]?.length) || stat.Id
  ); // they all start with '/'
  return objFrom2Arrays(
    statusNames,
    statusAr.map((stat, i) => ({
      name: statusNames[i],
      Status: stat.Status,
      State: stat.State,
    }))
  );
}

export async function getContainerStatus(
  dockerode: Dockerode,
  name: string
): Promise<ContainerStatus | "not deployed"> {
  const status = (await dockerode.listContainers({ all: true })).filter(
    ({ Names }) => Names[0] === "/" + name
  );
  return status[0]
    ? {
        State: status[0].State as "running" | "created" | "exited",
        Status: status[0].Status,
        name,
      }
    : "not deployed";
}

export async function getContainerLog(containerName: string, logTail?: number) {
  return (
    await execute(
      `docker logs ${containerName}${logTail ? ` --tail ${logTail}` : ""}`
    )
  ).log;
}

export async function startContainer(containerName: string) {
  return await execute(`docker start ${containerName}`);
}

export async function stopContainer(containerName: string) {
  return await execute(`docker stop ${containerName}`);
}

export async function deleteContainer(containerName: string) {
  return await execute(
    `docker stop ${containerName} && docker container rm ${containerName}`
  );
}

/* Docker Build */

export async function dockerBuild(
  imageName: string,
  { buildPath, dockerfilePath }: DockerBuildArgs,
  repoPath: string,
  username?: string,
  password?: string
) {
  if (username && password) {
    await execute(`docker login -u ${username} -p ${password}`);
  }
  const cd = `cd ${join(repoPath, imageName, buildPath)}`;

  const build = `docker build -t ${join(username || "", imageName)}${
    dockerfilePath ? ` -f ${dockerfilePath}` : ""
  } .`;

  const push = `docker push ${join(username || "", imageName)}`;

  return await execute(`${cd} && ${build} && ${push}`);
}

/* Docker Run */

export async function dockerRun(
  {
    image,
    ports,
    environment,
    network,
    volumes,
    restart,
    postImage,
    containerName,
    containerUser,
  }: DockerRunArgs,
  sysRoot: string,
  repoMount?: { repoFolder: string; containerMount: string },
  username?: string,
  password?: string
) {
  if (username && password) {
    await execute(`docker login -u ${username} -p ${password}`);
  }

  const command =
    `docker pull ${image} && docker run -d` +
    name(containerName) +
    containerUserString(containerUser) +
    portsString(ports) +
    volsString(sysRoot, volumes) +
    repoVolume(containerName, repoMount) +
    envString(environment) +
    restartString(restart) +
    networkString(network) +
    ` ${image}${postImage ? " " + postImage : ""}`;

  return await execute(command);
}

function name(containerName: string) {
  return containerName ? ` --name ${containerName}` : "";
}

function portsString(ports?: Conversion[]) {
  return ports && ports.length > 0
    ? ports
        .map(({ local, container }) => ` -p ${local}:${container}`)
        .reduce((prev, curr) => prev + curr)
    : "";
}

// function volsString(folderName: string, sysRoot: string, volumes?: Volume[]) {
//   return volumes && volumes.length > 0
//     ? volumes
//         .map(({ local, container, useSystemRoot }) => {
//           const mid = useSystemRoot ? "" : `${folderName}/`;
//           const localString =
//             local.length > 0
//               ? local[0] === "/"
//                 ? local.slice(1, local.length)
//                 : local
//               : "";
//           return ` -v ${sysRoot + mid + localString}:${container}`;
//         })
//         .reduce((prev, curr) => prev + curr)
//     : "";
// }

function volsString(sysRoot: string, volumes?: Conversion[]) {
  return volumes && volumes.length > 0
    ? volumes
        .map(({ local, container }) => {
          return ` -v ${local.replace("~/", sysRoot)}:${container}`;
        })
        .reduce((prev, curr) => prev + curr)
    : "";
}

function repoVolume(
  containerName?: string,
  repoMount?: { repoFolder: string; containerMount: string }
) {
  // repo root should be SYSROOT + "repos/"

  return repoMount
    ? ` -v ${join(repoMount.repoFolder, containerName)}:${
        repoMount.containerMount
      }`
    : "";
}

function restartString(restart?: string) {
  return restart
    ? ` --restart ${restart}${restart === "on-failure" ? ":10" : ""}`
    : "";
}

function envString(environment?: EnvironmentVar[]) {
  return environment && environment.length > 0
    ? environment
        .map(({ variable, value }) => ` -e "${variable}=${value}"`)
        .reduce((prev, curr) => prev + curr)
    : "";
}

function networkString(network?: string) {
  return network ? ` --network ${network}` : "";
}

function containerUserString(containerUser?: string) {
  return containerUser && containerUser.length > 0
    ? ` -u ${containerUser}`
    : "";
}
