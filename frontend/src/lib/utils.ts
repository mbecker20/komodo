import { ResourceComponents } from "@components/resources";
import { Types } from "@monitor/client";
import { UsableResource } from "@types";
import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export const object_keys = <T extends object>(o: T): (keyof T)[] =>
  Object.keys(o) as (keyof T)[];

export const RESOURCE_TARGETS: UsableResource[] = [
  "Deployment",
  "Server",
  "Build",
  "Procedure",
  "Repo",
  "Builder",
  "Alerter",
];

export function env_to_text(envVars: Types.EnvironmentVar[] | undefined) {
  return envVars?.reduce(
    (prev, { variable, value }) =>
      prev + (prev ? "\n" : "") + `${variable}=${value}`,
    ""
  );
}

export function text_to_env(env: string): Types.EnvironmentVar[] {
  return env
    .split("\n")
    .filter((line) => keep_line(line))
    .map((entry) => {
      const [first, ...rest] = entry.replaceAll('"', "").split("=");
      return [first, rest.join("=")];
    })
    .map(([variable, value]) => ({ variable, value }));
}

function keep_line(line: string) {
  if (line.length === 0) return false;
  let firstIndex = -1;
  for (let i = 0; i < line.length; i++) {
    if (line[i] !== " ") {
      firstIndex = i;
      break;
    }
  }
  if (firstIndex === -1) return false;
  if (line[firstIndex] === "#") return false;
  return true;
}

export function version_is_none({ major, minor, patch }: Types.Version) {
  return major === 0 && minor === 0 && patch === 0;
}

export function resource_name(type: UsableResource, id: string) {
  const Components = ResourceComponents[type];
  return Components.name(id);
}

export const level_to_number = (level: Types.PermissionLevel | undefined) => {
  switch (level) {
    case undefined:
      return 0;
    case Types.PermissionLevel.None:
      return 0;
    case Types.PermissionLevel.Read:
      return 1;
    case Types.PermissionLevel.Execute:
      return 2;
    case Types.PermissionLevel.Write:
      return 3;
  }
};

export const has_minimum_permissions = (
  level: Types.PermissionLevel | undefined,
  greater_than: Types.PermissionLevel
) => {
  if (!level) return false;
  return level_to_number(level) >= level_to_number(greater_than);
};

const tzOffset = new Date().getTimezoneOffset() * 60;

export const convertTsMsToLocalUnixTsInSecs = (ts: number) =>
  ts / 1000 - tzOffset;

export const usableResourcePath = (resource: UsableResource) => {
  if (resource === "ServerTemplate") return "server-templates"
  return `${resource.toLowerCase()}s`
}
