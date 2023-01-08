import { DockerContainerState, EnvironmentVar, ServerStatus, Timelength, Version } from "../types";

export function combineClasses(...classes: (string | false | undefined)[]) {
  return classes.filter((c) => (c ? true : false)).join(" ");
}

export function inPx(num: number) {
  return `${num}px`;
}

export type QueryObject = Record<string, string | number | undefined>;

export function generateQuery(query?: QueryObject) {
  if (query) {
    const q = Object.keys(query)
      .filter((key) => query[key] !== undefined)
      .map((key) => key + "=" + query[key])
      .join("&");
    return q && `?${q}`;
  } else return "";
}

export function readableTimestamp(unix_time_ms: number) {
  const date = new Date(unix_time_ms);
  const hours24 = date.getHours();
  let hours = hours24 % 12;
  if (hours === 0) hours = 12;
  const pm = hours24 > 11;
  const minutes = date.getMinutes();
  return `${date.getMonth() + 1}/${date.getDate()} ${hours}:${
    minutes > 9 ? minutes : "0" + minutes
  } ${pm ? "PM" : "AM"}`;
}

export function readableMonitorTimestamp(rfc3339_ts: string) {
  const date = new Date(rfc3339_ts);
  const hours24 = date.getHours();
  let hours = hours24 % 12;
  if (hours === 0) hours = 12;
  const pm = hours24 > 11;
  const minutes = date.getMinutes();
  return `${date.getMonth() + 1}/${date.getDate()} ${hours}:${
    minutes > 9 ? minutes : "0" + minutes
  } ${pm ? "PM" : "AM"}`;
}

export function readableDuration(start_ts: string, end_ts: string) {
  const start = new Date(start_ts);
  const end = new Date(end_ts);
  const durr = end.getTime() - start.getTime();
  const seconds = (durr / 1000).toFixed(1);
  return `${seconds} seconds`;
}

const tzOffset = new Date().getTimezoneOffset() * 60;

export function convertTsMsToLocalUnixTsInSecs(ts: number) {
  return ts / 1000 - tzOffset;
}

export function validatePercentage(perc: string) {
  // validates that a string represents a percentage
  const percNum = Number(perc);
  return !isNaN(percNum) && percNum > 0 && percNum < 100;
}

export function filterOutFromObj<T>(obj: T, idsToFilterOut: string[]) {
  return Object.fromEntries(
    Object.entries(obj as any).filter((entry) => {
      return !idsToFilterOut.includes(entry[0]);
    })
  ) as T;
}

export function keepOnlyInObj<T>(obj: T, idsToKeep: string[]) {
  return Object.fromEntries(
    Object.entries(obj as any).filter((entry) => {
      return idsToKeep.includes(entry[0]);
    })
  ) as T;
}

export function getNestedEntry(obj: any, idPath: string[]): any {
  if (idPath.length === 0) {
    return obj;
  } else {
    return getNestedEntry(obj[idPath[0]], idPath.slice(1));
  }
}

export function intoCollection<T>(
  arr: T[],
  idPath: string[]
): Record<string, T> {
  return Object.fromEntries(
    arr.map((item) => [getNestedEntry(item, idPath), item])
  );
}

export function getId(entity: any): string {
  return entity._id.$oid;
}

export function deploymentStateClass(state: DockerContainerState) {
  switch (state) {
    case DockerContainerState.Running:
      return "green";
    case DockerContainerState.Exited:
      return "red";
    default:
      return "blue";
  }
}

export function serverStatusClass(status: ServerStatus) {
  if (status === ServerStatus.Ok) {
    return "running";
  } else if (status === ServerStatus.NotOk) {
    return "exited";
  }
}

export function copyToClipboard(text: string) {
  navigator.clipboard.writeText(text);
}

export function parseEnvVarseToDotEnv(envVars: EnvironmentVar[]) {
  return envVars.reduce(
    (prev, { variable, value }) =>
      prev + (prev ? "\n" : "") + `${variable}=${value}`,
    ""
  );
}

function shouldKeepLine(line: string) {
  if (line.length === 0) {
    return false;
  }
  let firstIndex = -1;
  for (let i = 0; i < line.length; i++) {
    if (line[i] !== " ") {
      firstIndex = i;
      break;
    }
  }
  if (firstIndex === -1) {
    return false;
  }
  if (line[firstIndex] === "#") {
    return false;
  }
  return true;
}

export function parseDotEnvToEnvVars(env: string): EnvironmentVar[] {
  return env
    .split("\n")
    .filter((line) => shouldKeepLine(line))
    .map((entry) => {
      const [first, ...rest] = entry.replaceAll('"', "").split("=");
      return [first, rest.join("=")];
    })
    .map(([variable, value]) => ({ variable, value }));
}

export function deploymentHeaderStateClass(
  state: DockerContainerState,
) {
  switch (state) {
    case DockerContainerState.Running:
      return "running";
    case DockerContainerState.Exited:
      return "exited";
  }
}

export function version_to_string(version: Version) {
  return `${version.major}.${version.minor}.${version.patch}`
}

export function string_to_version(version: string): Version {
  const [major, minor, patch] = version.split(".")
  return {
    major: Number(major),
    minor: Number(minor),
    patch: Number(patch),
  }
}

export function get_to_one_sec_divisor(timelength: Timelength) {
  // returns what the timelength needs to be divided to convert to per second values
  if (timelength === Timelength.OneSecond) {
    return 1
  } else if (timelength === Timelength.FiveSeconds) {
    return 5
  } else if (timelength === Timelength.ThirtySeconds) {
    return 30
  } else if (timelength === Timelength.OneMinute) {
    return 60
  }
}