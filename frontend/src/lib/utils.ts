import { ResourceComponents } from "@components/resources";
import { Types } from "@monitor/client";
import { UsableResource } from "@types";
import Convert from "ansi-to-html";
import { type ClassValue, clsx } from "clsx";
import sanitizeHtml from "sanitize-html";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export const object_keys = <T extends object>(o: T): (keyof T)[] =>
  Object.keys(o) as (keyof T)[];

export const RESOURCE_TARGETS: UsableResource[] = [
  "Server",
  "Deployment",
  "Build",
  "Repo",
  "Procedure",
  "Builder",
  "Alerter",
  "ServerTemplate",
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

export function version_is_none(version?: Types.Version) {
  if (!version) return true;
  return version.major === 0 && version.minor === 0 && version.patch === 0;
}

export function resource_name(type: UsableResource, id: string) {
  const Components = ResourceComponents[type];
  return Components.list_item(id)?.name;
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
  if (resource === "ServerTemplate") return "server-templates";
  return `${resource.toLowerCase()}s`;
};

export const sanitizeOnlySpan = (log: string) => {
  return sanitizeHtml(log, {
    allowedTags: ["span"],
    allowedAttributes: {
      span: ["class"],
    },
  });
};

const convert = new Convert();
/**
 * Converts the ansi colors in log to html.
 * sanitizes incoming log first for any eg. script tags.
 * @param log incoming log string
 */
export const logToHtml = (log: string) => {
  if (!log) return "No log.";
  const sanitized = sanitizeHtml(log, {
    allowedTags: sanitizeHtml.defaults.allowedTags.filter(
      (tag) => tag !== "script"
    ),
    allowedAttributes: sanitizeHtml.defaults.allowedAttributes,
  });
  return convert.toHtml(sanitized);
};

export const getUpdateQuery = (
  target: Types.ResourceTarget,
  deployments: Types.DeploymentListItem[] | undefined
) => {
  const build_id =
    target.type === "Deployment"
      ? deployments?.find((d) => d.id === target.id)?.info.build_id
      : undefined;
  if (build_id) {
    return {
      $or: [
        {
          "target.type": target.type,
          "target.id": target.id,
        },
        {
          "target.type": "Build",
          "target.id": build_id,
          operation: {
            $in: [Types.Operation.RunBuild, Types.Operation.CancelBuild],
          },
        },
      ],
    };
  } else {
    return {
      "target.type": target.type,
      "target.id": target.id,
    };
  }
};

export const filterBySplit = <T>(
  items: T[] | undefined,
  search: string,
  extract: (item: T) => string
) => {
  const split = search.toLowerCase().split(" ");
  return (
    (split.length
      ? items?.filter((item) => {
          const target = extract(item);
          return split.every((term) => target.includes(term));
        })
      : items) ?? []
  );
};
