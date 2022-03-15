import {
  Conversion,
  DockerBuildArgs,
  DockerRunArgs,
  EnvironmentVar,
  Volume,
} from "@monitor/types";
import { execute } from "./execute";

/* Docker Build */

export async function dockerBuild(
  { buildPath, dockerfilePath, imageName }: DockerBuildArgs,
  repoPath: string,
  registryUrl: string
) {
  const command = `cd ${repoPath}${imageName}${
    buildPath && (buildPath[0] === "/" ? buildPath : "/" + buildPath)
  } && docker build -t ${
    registryUrl + imageName
  } -f ${dockerfilePath} . && docker push ${registryUrl + imageName}`;
  return {
    command,
    ...(await execute(command)),
  };
}

/* Docker Run */

export async function dockerRun(
  {
    image,
    latest,
    ports,
    environment,
    network,
    volumes,
    restart,
    postImage,
    containerName,
    containerUser,
  }: DockerRunArgs,
  sysRoot: string
) {
  const command =
    `docker pull ${image}${latest && ":latest"} && ` +
    `docker run -d --name ${containerName}` +
    containerUserString(containerUser) +
    portsString(ports) +
    volsString(containerName!, sysRoot, volumes) +
    envString(environment) +
    restartString(restart) +
    networkString(network) +
    ` ${image}${latest && ":latest"}${postImage && " " + postImage}`;

  return {
    command,
    ...(await execute(command)),
  };
}

function portsString(ports?: Conversion[]) {
  return ports && ports.length > 0
    ? ports
        .map(({ local, container }) => ` -p ${local}:${container}`)
        .reduce((prev, curr) => prev + curr)
    : "";
}

function volsString(folderName: string, sysRoot: string, volumes?: Volume[]) {
  return volumes && volumes.length > 0
    ? volumes
        .map(({ local, container, useSystemRoot }) => {
          const mid = !useSystemRoot && `${folderName}/`;
          const localString =
            local.length > 0
              ? local[0] === "/"
                ? local.slice(1, local.length)
                : local
              : "";
          return ` -v ${sysRoot + mid + localString}:${container}`;
        })
        .reduce((prev, curr) => prev + curr)
    : "";
}

function restartString(restart?: string) {
  return restart
    ? ` --restart=${restart}${restart === "on-failure" ? ":10" : ""}`
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
  return network ? ` --network=${network}` : "";
}

function containerUserString(containerUser?: string) {
  return containerUser && containerUser.length > 0
    ? ` -u ${containerUser}`
    : "";
}
