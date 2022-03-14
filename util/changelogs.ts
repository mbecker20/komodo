import { Build, Conversion, Deployment, EnvironmentVar, Server, Volume } from "@monitor/types";
import { objFrom2Arrays } from "./helpers";

const deploymentViewFields = [
  "name",
  "image",
  "buildID",
  "ports",
  "volumes",
  "environment",
  "network",
  "useServerRoot",
  "restart",
  "latest",
  "logTail",
  "autoDeploy",
  "containerUser",
  "postImage",
]; // the fields shown in the update log

export function deploymentChangelog(
  oldDeployment: Deployment,
  newDeployment: Deployment
) {
  const changelogArray = deploymentViewFields
    .filter((field) => {
      return (
        (newDeployment[field] || oldDeployment[field]) &&
        newDeployment[field] !== oldDeployment[field]
      );
    })
    .map((field) => {
      switch (field) {
        case "environment":
          return envChangelog(
            oldDeployment.environment,
            newDeployment.environment
          );
        case "volumes":
          return volumesChangelog(oldDeployment.volumes, newDeployment.volumes);
        case "ports":
          return portsChangelog(oldDeployment.ports, newDeployment.ports);
        default:
          return `${field}: ${JSON.stringify(
            oldDeployment[field]
          )} -> ${JSON.stringify(newDeployment[field])}, \n\n`;
      }
    });
  return changelogArray.length > 0
    ? "Changelog:\n\n" + changelogArray.reduce((prev, curr) => prev + curr)
    : "No Changes";
}

function envChangelog(oldEnv?: EnvironmentVar[], newEnv?: EnvironmentVar[]) {
  if (!oldEnv) {
    if (!newEnv) {
      return "";
    } else {
      return `Added Environment:\n${newEnv
        .map((env) => `\t${env.variable}: ${env.value}, \n`)
        .reduce((prev, curr) => prev + curr)}`;
    }
  } else if (!newEnv) {
    return `Removed Environment:\n${oldEnv
      .map((env) => `\t${env.variable}: ${env.value}, \n`)
      .reduce((prev, curr) => prev + curr)}`;
  } else {
    const oldVars = oldEnv.map((env) => env.variable);
    const oldVals = oldEnv.map((env) => env.value);
    const newVars = newEnv.map((env) => env.variable);
    const newVals = newEnv.map((env) => env.value);
    const oldObj = objFrom2Arrays(oldVars, oldVals);
    const newObj = objFrom2Arrays(newVars, newVals);
    const additions: string[] = [];
    const changes: string[] = [];
    const deletions: string[] = [];
    oldVars.forEach((variable) => {
      if (newVars.includes(variable)) {
        if (oldObj[variable] !== newObj[variable]) changes.push(variable);
      } else {
        deletions.push(variable);
      }
    });
    newVars.forEach((variable) => {
      if (!oldVars.includes(variable)) {
        additions.push(variable);
      }
    });

    const additionsString =
      additions.length > 0
        ? `\tAdditions:\n` +
          additions
            .map((addition) => `\t\t${addition}: ${newObj[addition]}, \n`)
            .reduce((prev, curr) => prev + curr)
        : "";
    const changesString =
      changes.length > 0
        ? `\tChanges:\n` +
          changes
            .map(
              (change) =>
                `\t\t${change}: ${oldObj[change]} -> ${newObj[change]}, \n`
            )
            .reduce((prev, curr) => prev + curr)
        : "";
    const deletionsString =
      deletions.length > 0
        ? `\tDeletions:\n` +
          deletions
            .map((deletion) => `\t\t${deletion}: ${oldObj[deletion]}, \n`)
            .reduce((prev, curr) => prev + curr)
        : "";
    const show =
      additionsString.length > 0 ||
      changesString.length > 0 ||
      deletionsString.length > 0;
    return show
      ? "Environment:\n" + additionsString + changesString + deletionsString
      : "";
  }
}

function portsChangelog(oldPorts?: Conversion[], newPorts?: Conversion[]) {
  if (!oldPorts) {
    if (!newPorts) {
      return "";
    } else {
      return `Added Ports:\n${newPorts
        .map((port) => `\t${port.local}: ${port.container}, \n`)
        .reduce((prev, curr) => prev + curr)}`;
    }
  } else if (!newPorts) {
    return `Removed Ports:\n${oldPorts
      .map((port) => `\t${port.local}: ${port.container}, \n`)
      .reduce((prev, curr) => prev + curr)}`;
  } else {
    const oldLocal = oldPorts.map((env) => env.local);
    const oldContainer = oldPorts.map((env) => env.container);
    const newLocal = newPorts.map((env) => env.local);
    const newContainer = newPorts.map((env) => env.container);
    const oldObj = objFrom2Arrays(oldLocal, oldContainer);
    const newObj = objFrom2Arrays(newLocal, newContainer);
    const additions: string[] = [];
    const changes: string[] = [];
    const deletions: string[] = [];
    oldLocal.forEach((local) => {
      if (newLocal.includes(local)) {
        if (oldObj[local] !== newObj[local]) changes.push(local);
      } else {
        deletions.push(local);
      }
    });
    newLocal.forEach((local) => {
      if (!oldLocal.includes(local)) {
        additions.push(local);
      }
    });

    const additionsString =
      additions.length > 0
        ? `\tAdditions:\n` +
          additions
            .map((addition) => `\t\t${addition}: ${newObj[addition]}, \n`)
            .reduce((prev, curr) => prev + curr)
        : "";
    const changesString =
      changes.length > 0
        ? `\tChanges:\n` +
          changes
            .map(
              (change) =>
                `\t\t${change}: ${oldObj[change]} -> ${newObj[change]}, \n`
            )
            .reduce((prev, curr) => prev + curr)
        : "";
    const deletionsString =
      deletions.length > 0
        ? `\tDeletions:\n` +
          deletions
            .map((deletion) => `\t\t${deletion}: ${oldObj[deletion]}, \n`)
            .reduce((prev, curr) => prev + curr)
        : "";
    const show =
      additionsString.length > 0 ||
      changesString.length > 0 ||
      deletionsString.length > 0;
    return show
      ? "Ports:\n" + additionsString + changesString + deletionsString
      : "";
  }
}

function volumesChangelog(oldVols?: Volume[], newVols?: Volume[]) {
  if (!oldVols) {
    if (!newVols) {
      return "";
    } else {
      return `Added Volumes:\n${newVols
        .map(
          (vol) =>
            `\t${vol.local}: ${vol.container}, Use Filesystem Root: ${
              vol.useSystemRoot ? "true" : "false"
            }, \n`
        )
        .reduce((prev, curr) => prev + curr)}`;
    }
  } else if (!newVols) {
    return `Removed Volumes:\n${oldVols
      .map(
        (vol) =>
          `\t${vol.local}: ${vol.container}, Use Filesystem Root: ${
            vol.useSystemRoot ? "true" : "false"
          }, \n`
      )
      .reduce((prev, curr) => prev + curr)}`;
  } else {
    const oldLocal = oldVols.map((env) => env.local);
    const oldContainer = oldVols.map((env) => env.container);
    const oldUseSystem = oldVols.map((env) => env.useSystemRoot);
    const newLocal = newVols.map((env) => env.local);
    const newContainer = newVols.map((env) => env.container);
    const newUseSystem = newVols.map((env) => env.useSystemRoot);
    const oldObj = objFrom2Arrays(oldLocal, oldContainer);
    const oldObjSystem = objFrom2Arrays(oldLocal, oldUseSystem);
    const newObj = objFrom2Arrays(newLocal, newContainer);
    const newObjSystem = objFrom2Arrays(newLocal, newUseSystem);
    const additions: string[] = [];
    const changes: string[] = [];
    const deletions: string[] = [];
    oldLocal.forEach((local) => {
      if (newLocal.includes(local)) {
        if (
          oldObj[local] !== newObj[local] ||
          oldObjSystem[local] !== newObjSystem[local]
        )
          changes.push(local);
      } else {
        deletions.push(local);
      }
    });
    newLocal.forEach((local) => {
      if (!oldLocal.includes(local)) {
        additions.push(local);
      }
    });

    const additionsString =
      additions.length > 0
        ? `\tAdditions:\n` +
          additions
            .map(
              (addition) =>
                `\t\t${addition}: ${newObj[addition]}, Use Filesystem Root: ${
                  newObjSystem[addition] ? "true" : false
                }, \n`
            )
            .reduce((prev, curr) => prev + curr)
        : "";
    const changesString =
      changes.length > 0
        ? `\tChanges:\n` +
          changes
            .map(
              (change) =>
                `\t\t${change}: ${oldObj[change]} -> ${newObj[change]}${
                  oldObjSystem[change] !== newObjSystem[change]
                    ? `, Use Filesystem Root: ${
                        oldObjSystem[change] ? "true" : "false"
                      } -> ${newObjSystem[change] ? "true" : "false"}`
                    : ""
                } \n`
            )
            .reduce((prev, curr) => prev + curr)
        : "";
    const deletionsString =
      deletions.length > 0
        ? `\tDeletions:\n` +
          deletions
            .map(
              (deletion) =>
                `\t\t${deletion}: ${oldObj[deletion]}, Use Filesystem Root: ${
                  oldObjSystem[deletion] ? "true" : "false"
                }, \n`
            )
            .reduce((prev, curr) => prev + curr)
        : "";
    const show =
      additionsString.length > 0 ||
      changesString.length > 0 ||
      deletionsString.length > 0;
    return show
      ? "Volumes:\n" + additionsString + changesString + deletionsString
      : "";
  }
}

const buildViewFields = [
  "name",
  "repoURL",
  "repoName",
  "branch",
  "buildPath",
  "dockerfilePath",
  "owner",
]; // the fields shown in the update log

export function buildChangelog(oldBuild: Build, newBuild: Build) {
  const changelogArray = buildViewFields
    .filter((field) => {
      return (
        (newBuild[field] || oldBuild[field]) &&
        newBuild[field] !== oldBuild[field]
      );
    })
    .map((field) => {
      return `${field}: ${JSON.stringify(oldBuild[field])} -> ${JSON.stringify(
        newBuild[field]
      )}, \n\n`;
    });
  return changelogArray.length > 0
    ? "Changelog:\n\n" + changelogArray.reduce((prev, curr) => prev + curr)
    : "No Changes";
}

const serverViewFields = ["name", "address", "port", "enabled"];

export function serverChangelog(oldServer: Server, newServer: Server) {
  const changelogArray = serverViewFields
    .filter((field) => {
      return (
        (newServer[field] || oldServer[field]) &&
        newServer[field] !== oldServer[field]
      );
    })
    .map((field) => {
      return `${field}: ${JSON.stringify(oldServer[field])} -> ${JSON.stringify(
        newServer[field]
      )}, \n\n`;
    });
  return changelogArray.length > 0
    ? "Changelog:\n\n" + changelogArray.reduce((prev, curr) => prev + curr)
    : "No Changes";
}
