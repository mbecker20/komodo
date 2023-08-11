/* eslint-disable @typescript-eslint/no-explicit-any */
import { Types } from "@monitor/client";
import sanitizeHtml from "sanitize-html";

import { clsx, type ClassValue } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export function combineClasses(...classes: (string | false | undefined)[]) {
  return classes.filter((c) => (c ? true : false)).join(" ");
}

export function inPx(num: number) {
  return `${num}px`;
}

export type QueryObject = Record<string, string | number | boolean | undefined>;

export function sanitizeLog(log: string) {
  return sanitizeHtml(log, {
    allowedTags: sanitizeHtml.defaults.allowedTags.filter(
      (tag) => tag !== "script"
    ),
    allowedAttributes: sanitizeHtml.defaults.allowedAttributes,
  });
}

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

export function readableDuration(start_ts: number, end_ts: number) {
  const start = new Date(start_ts);
  const end = new Date(end_ts);
  const durr = end.getTime() - start.getTime();
  const seconds = durr / 1000;
  const minutes = Math.floor(seconds / 60);
  const remaining_seconds = seconds % 60;
  return `${
    minutes > 0 ? `${minutes} minute${minutes > 1 ? "s" : ""} ` : ""
  }${remaining_seconds.toFixed(minutes > 0 ? 0 : 1)} seconds`;
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

export function deploymentStateClass(state: Types.DockerContainerState) {
  switch (state) {
    case Types.DockerContainerState.Running:
      return "green";
    case Types.DockerContainerState.Exited:
      return "red";
    default:
      return "blue";
  }
}

export function serverStatusClass(status: Types.ServerStatus) {
  if (status === Types.ServerStatus.Ok) {
    return "running";
  } else if (status === Types.ServerStatus.NotOk) {
    return "exited";
  }
}

export function copyToClipboard(text: string) {
  navigator.clipboard.writeText(text);
}

export function parseEnvVarseToDotEnv(
  envVars: Types.EnvironmentVar[] | undefined
) {
  return envVars?.reduce(
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

export function parseDotEnvToEnvVars(env: string): Types.EnvironmentVar[] {
  return env
    .split("\n")
    .filter((line) => shouldKeepLine(line))
    .map((entry) => {
      const [first, ...rest] = entry.replaceAll('"', "").split("=");
      return [first, rest.join("=")];
    })
    .map(([variable, value]) => ({ variable, value }));
}

export function deploymentHeaderStateClass(state: Types.DockerContainerState) {
  switch (state) {
    case Types.DockerContainerState.Running:
      return "running";
    case Types.DockerContainerState.Exited:
      return "exited";
  }
}

export const keys = <T extends object>(o: T): (keyof T)[] =>
  Object.keys(o) as (keyof T)[];

export function version_to_string(version: Types.Version | undefined) {
  if (!version) return;
  if (!keys(version).some((k) => !!version[k])) return;
  return `v${version.major}.${version.minor}.${version.patch}`;
}

export function string_to_version(version: string): Types.Version {
  const [major, minor, patch] = version.split(".");
  return {
    major: Number(major),
    minor: Number(minor),
    patch: Number(patch),
  };
}

export function get_to_one_sec_divisor(timelength: Types.Timelength) {
  // returns what the timelength needs to be divided to convert to per second values
  if (timelength === Types.Timelength.OneSecond) {
    return 1;
  } else if (timelength === Types.Timelength.FiveSeconds) {
    return 5;
  } else if (timelength === Types.Timelength.ThirtySeconds) {
    return 30;
  } else if (timelength === Types.Timelength.OneMinute) {
    return 60;
  }
}

export function convert_timelength_to_ms(timelength: Types.Timelength) {
  // returns what the timelength needs to be divided to convert to per second values
  if (timelength === Types.Timelength.OneSecond) {
    return 1000;
  } else if (timelength === Types.Timelength.FiveSeconds) {
    return 5000;
  } else if (timelength === Types.Timelength.ThirtySeconds) {
    return 30000;
  } else if (timelength === Types.Timelength.OneMinute) {
    return 60000;
  }
}

export function readableStorageAmount(gb: number) {
  if (gb > 512) {
    return `${(gb / 1024).toFixed(1)} TB`;
  } else if (gb < 1) {
    return `${(gb * 1024).toFixed()} MiB`;
  } else {
    return `${gb.toFixed()} GiB`;
  }
}

export function readableVersion(version: Types.Version) {
  if (version.major === 0 && version.minor === 0 && version.patch === 0)
    return "latest";
  return `v${version.major}.${version.minor}.${version.patch}`;
}

export function readableUserType(user: Types.User) {
  if (user.github_id) {
    return "github";
  } else if (user.google_id) {
    return "google";
  } else {
    return "local";
  }
}

const KB = 1024;
const MB = 1024 ** 2;
const GB = 1024 ** 3;

export function readableBytes(bytes: number) {
  if (bytes > GB) {
    return `${(bytes / GB).toFixed(1)} GiB`;
  } else if (bytes > MB) {
    return `${(bytes / MB).toFixed(1)} MiB`;
  } else {
    return `${(bytes / KB).toFixed(1)} KiB`;
  }
}

export function readableImageId(id: string) {
  return id.replaceAll("sha256:", "").slice(0, 12);
}

export function readableImageNameTag(
  repoTags?: string[],
  repoDigests?: string[]
): [string, string] {
  if (repoTags && repoTags.length > 0) {
    const [name, tag] = repoTags[0].split(":");
    return [name, tag];
  } else if (repoDigests && repoDigests.length > 0) {
    const [name] = repoDigests[0].split("@");
    return [name, "none"];
  } else {
    return ["none", "none"];
  }
}

export const fmt_update_date = (d: Date) =>
  `${d.getDate()}/${d.getMonth() + 1} @ ${d.getHours()}:${d.getMinutes()}`;
