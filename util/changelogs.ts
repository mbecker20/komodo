import {
  Build,
  Conversion,
  Deployment,
  EnvironmentVar,
  Server,
} from "@monitor/types";
import { objFrom2Arrays, prettyStringify } from "./helpers";

const deploymentViewFields: (keyof Deployment)[] = [
  "name",
  "image",
  "buildID",
  "ports",
  "volumes",
  "environment",
  "network",
  "restart",
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
          return `${field}: ${prettyStringify(
            oldDeployment[field]
          )} -> ${prettyStringify(newDeployment[field])}, \n\n`;
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

function volumesChangelog(oldVols?: Conversion[], newVols?: Conversion[]) {
  if (!oldVols) {
    if (!newVols) {
      return "";
    } else {
      return `Added Volumes:\n${newVols
        .map((vol) => `\t${vol.local}: ${vol.container},\n`)
        .reduce((prev, curr) => prev + curr)}`;
    }
  } else if (!newVols) {
    return `Removed Volumes:\n${oldVols
      .map((vol) => `\t${vol.local}: ${vol.container},\n`)
      .reduce((prev, curr) => prev + curr)}`;
  } else {
    const oldLocal = oldVols.map((vol) => vol.local);
    const oldContainer = oldVols.map((vol) => vol.container);
    const newLocal = newVols.map((vol) => vol.local);
    const newContainer = newVols.map((vol) => vol.container);
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
          .map((addition) => `\t\t${addition}: ${newObj[addition]},\n`)
          .reduce((prev, curr) => prev + curr)
        : "";
    const changesString =
      changes.length > 0
        ? `\tChanges:\n` +
        changes
          .map(
            (change) =>
              `\t\t${change}: ${oldObj[change]} -> ${newObj[change]}\n`
          )
          .reduce((prev, curr) => prev + curr)
        : "";
    const deletionsString =
      deletions.length > 0
        ? `\tDeletions:\n` +
        deletions
          .map((deletion) => `\t\t${deletion}: ${oldObj[deletion]},\n`)
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

const buildViewFields: (keyof Build)[] = [
  "name",
  "repo",
  "branch",
  "onClone",
  "cliBuild",
  "dockerBuildArgs",
]; // the fields shown in the update log

export function buildChangelog(oldBuild: Build, newBuild: Build) {
  const changelogArray = buildViewFields
    .filter((field) => {
      return (
        (newBuild[field] || oldBuild[field]) &&
        prettyStringify(newBuild[field]) !== prettyStringify(oldBuild[field])
      );
    })
    .map((field) => {
      return `${field}: ${prettyStringify(oldBuild[field])} -> ${prettyStringify(
        newBuild[field]
      )}, \n\n`;
    });
  return changelogArray.length > 0
    ? "Changelog:\n\n" + changelogArray.reduce((prev, curr) => prev + curr)
    : "No Changes";
}

const serverViewFields = ["name", "address", "port", "enabled", "region", "toNotify", "cpuAlert", "memAlert", "diskAlert"];

export function serverChangelog(oldServer: Server, newServer: Server) {
  const changelogArray = serverViewFields
    .filter((field) => {
      return (
        (newServer[field] || oldServer[field]) &&
        newServer[field] !== oldServer[field]
      );
    })
    .map((field) => {
      return `${field}: ${prettyStringify(oldServer[field])} -> ${prettyStringify(
        newServer[field]
      )}, \n\n`;
    });
  return changelogArray.length > 0
    ? "Changelog:\n\n" + changelogArray.reduce((prev, curr) => prev + curr)
    : "No Changes";
}
