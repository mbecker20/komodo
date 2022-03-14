import { Build, Conversion, Deployment, EnvironmentVar, Volume } from "@monitor/types";
import { FastifyInstance } from "fastify";
import { DEPLOYDATA_ROOT, REGISTRY_URL, SYSROOT } from "../../config";

export async function createDockerRun(
	app: FastifyInstance,
  {
    buildID,
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
  }: Deployment,
) {
  const _image = buildID
    ? REGISTRY_URL +
      ((await app.builds.findById(buildID)) as any as Build).imageName
    : image;
  return (
    `docker pull ${_image}${buildID || latest ? ":latest" : ""} && ` +
    `docker run -d --name ${containerName}` +
    containerUserString(containerUser) +
    portsString(ports) +
    volsString(containerName!, volumes) +
    envString(environment) +
    restartString(restart) +
    networkString(network) +
    ` ${_image}${buildID || latest ? ":latest" : ""}${
      postImage ? " " + postImage : ""
    }`
  );
}

function portsString(ports?: Conversion[]) {
  return ports && ports.length > 0
    ? ports
        .map(({ local, container }) => ` -p ${local}:${container}`)
        .reduce((prev, curr) => prev + curr)
    : "";
}

function volsString(folderName: string, volumes?: Volume[]) {
  return volumes && volumes.length > 0
    ? volumes
        .map(({ local, container, useSystemRoot }) => {
          const mid = useSystemRoot ? "" : `${DEPLOYDATA_ROOT}${folderName}/`;
          const localString =
            local.length > 0
              ? local[0] === "/"
                ? local.slice(1, local.length)
                : local
              : "";
          return ` -v ${SYSROOT + mid + localString}:${container}`;
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
