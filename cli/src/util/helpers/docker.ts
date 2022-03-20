import { CommandLogError } from "@monitor/types";
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
  onCommandEnd({
    stage: `${
      update.isError ? "error updating" : "updated"
    } system (1 of ${total})`,
    log: update,
  });
  if (update.isError)
    return {
      stage: "error updating system",
      log: update,
    };

  const installDeps = await execute(`
		sudo apt-get install \
    ca-certificates \
    curl \
    gnupg \
    lsb-release -y
	`);
  onCommandEnd({
    stage: `${
      installDeps.isError ? "error installing" : "installed"
    } dependencies (2 of ${total})`,
    log: installDeps,
  });
  if (installDeps.isError)
    return {
      stage: "error installing dependencies",
      log: installDeps,
    };

  const addKey = await execute(
    "curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg"
  );
  onCommandEnd({
    stage: `${
      addKey.isError ? "error adding" : "added"
    } docker key (3 of ${total})`,
    log: addKey,
  });
  if (addKey.isError)
    return {
      stage: "error adding docker key",
      log: addKey,
    };

  const setStableRepository = await execute(`
		echo \
  	"deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu \
  	$(lsb_release -cs) stable" | sudo tee /etc/apt/sources.list.d/docker.list > /dev/null
	`);
  onCommandEnd({
    stage: `${
      setStableRepository.isError ? "error setting" : "set"
    } docker stable repository (4 of ${total})`,
    log: setStableRepository,
  });
  if (setStableRepository.isError)
    return {
      stage: "error adding stable repository",
      log: setStableRepository,
    };

  const installDocker = await execute(
    "sudo apt-get udpate && sudo apt-get install docker-ce docker-ce-cli containerd.io -y"
  );
  onCommandEnd({
    stage: `${
      installDocker.isError ? "error installing" : "installed"
    } docker (5 of ${total})`,
    log: installDocker,
  });
  if (installDocker.isError)
    return {
      stage: "error installing docker",
      log: installDocker,
    };

  const addUser = await execute(
    "sudo groupadd docker && sudo usermod -aG docker $USER && newgrp docker"
  );
  onCommandEnd({
    stage: `${
      addUser.isError ? "error adding" : "added"
    } user to docker user group (6 of ${total})`,
    log: addUser,
  });
  if (addUser.isError)
    return {
      stage: "error adding user to docker group",
      log: addUser,
    };

  if (systemCtlEnable) {
    const startOnBoot = await execute(
      "sudo systemctl enable docker.service && sudo systemctl enable containerd.service"
    );
    onCommandEnd({
      stage: `${
        startOnBoot.isError ? "error configuring" : "configured"
      } to start on boot (7 of ${total})`,
      log: startOnBoot,
    });
    if (startOnBoot.isError)
      return {
        stage: "error configuring to start on boot",
        log: startOnBoot,
      };
  }

  return;
}

export async function isDockerInstalled() {
  const res = await execute("docker ps");
  return !res.isError;
}
