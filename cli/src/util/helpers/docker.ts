import { CommandLogError, Conversion, DockerRunArgs, EnvironmentVar, Volume } from "@monitor/types";
import { execute } from "./execute";

export type InstallLog = {
  stage: string;
  log: CommandLogError;
};

export async function installDockerUbuntu(
  onCommandEnd: (log: InstallLog) => void,
  systemCtlEnable?: boolean
) {
  const total = 6 + (systemCtlEnable ? 1 : 0);
  const update = await execute("sudo apt-get update");
  console.log(update);
  onCommandEnd({
    stage: `${
      update.isError ? "error updating" : "updated"
    } system (1 of ${total})`,
    log: update,
  });
  if (update.isError) return true;

  const installDeps = await execute(`sudo apt-get install \
    ca-certificates \
    curl \
    gnupg \
    lsb-release`);
  console.log(installDeps);
  onCommandEnd({
    stage: `${
      installDeps.isError ? "error installing" : "installed"
    } dependencies (2 of ${total})`,
    log: installDeps,
  });
  if (installDeps.isError) return true;

  const addKey = await execute(
    "curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg"
  );
  console.log(addKey);
  onCommandEnd({
    stage: `${
      addKey.isError ? "error adding" : "added"
    } docker key (3 of ${total})`,
    log: addKey,
  });
  if (addKey.isError) return true;

  const setStableRepository = await execute(`echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu \
  $(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null`);
  console.log(setStableRepository);
  onCommandEnd({
    stage: `${
      setStableRepository.isError ? "error setting" : "set"
    } docker stable repository (4 of ${total})`,
    log: setStableRepository,
  });
  if (setStableRepository.isError) return;

  const installDocker = await execute(
    "sudo apt-get udpate && sudo apt-get install docker-ce docker-ce-cli containerd.io -y"
  );
  console.log(installDocker);
  onCommandEnd({
    stage: `${
      installDocker.isError ? "error installing" : "installed"
    } docker (5 of ${total})`,
    log: installDocker,
  });
  if (installDocker.isError) return true;

  const addUser = await execute(
    "sudo groupadd docker && sudo usermod -aG docker $USER && newgrp docker"
  );
  console.log(addUser)
  onCommandEnd({
    stage: `${
      addUser.isError ? "error adding" : "added"
    } user to docker user group (6 of ${total})`,
    log: addUser,
  });
  if (addUser.isError) return true;

  if (systemCtlEnable) {
    const startOnBoot = await execute(
      "sudo systemctl enable docker.service && sudo systemctl enable containerd.service"
    );
    console.log(startOnBoot);
    onCommandEnd({
      stage: `${
        startOnBoot.isError ? "error configuring" : "configured"
      } to start on boot (7 of ${total})`,
      log: startOnBoot,
    });
    if (startOnBoot.isError) return true;
  }
}

export async function isDockerInstalled() {
  const res = await execute("docker ps");
  return !res.isError;
}

export async function deleteContainer(containerName: string) {
  return await execute(
    `docker stop ${containerName} && docker container rm ${containerName}`
  );
}

/* Docker Run for Deployments */
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
  }: DockerRunArgs
) {
  const command =
    `docker run -d` +
    name(containerName) +
    containerUserString(containerUser) +
    portsString(ports) +
    volsString(volumes) +
    envString(environment) +
    restartString(restart) +
    networkString(network) +
    ` ${image}${postImage ? " " + postImage : ""}`;

  return await execute(command);
}

function name(containerName?: string) {
  return containerName ? ` --name ${containerName}` : "";
}

function portsString(ports?: Conversion[]) {
  return ports && ports.length > 0
    ? ports
        .map(({ local, container }) => ` -p ${local}:${container}`)
        .reduce((prev, curr) => prev + curr)
    : "";
}

function volsString(volumes?: Volume[]) {
  return volumes && volumes.length > 0
    ? volumes
        .map(({ local, container }) => {
          return ` -v ${local}:${container}`;
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
  return network ? ` --network ${network}` : "";
}

function containerUserString(containerUser?: string) {
  return containerUser && containerUser.length > 0
    ? ` -u ${containerUser}`
    : "";
}