import { Types } from "@monitor/client";
import { UsableResource } from "@types";
import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs));
}

export const keys = <T extends object>(o: T): (keyof T)[] =>
  Object.keys(o) as (keyof T)[];

export const RESOURCE_TARGETS: UsableResource[] = [
  "Procedure",
  "Deployment",
  "Server",
  "Build",
  "Repo",
  "Builder",
  "Alerter",
];

export const fmt_date = (d: Date) => {
  return `${d.getDate()}/${
    d.getMonth() + 1
  } @ ${d.getHours()}:${d.getMinutes()}`;
};

export const fmt_date_with_minutes = (d: Date) => {
  // return `${d.toLocaleDateString()} ${d.toLocaleTimeString()}`;
  return d.toLocaleString();
};

export const fmt_version = (version: Types.Version | undefined) => {
  if (!version) return "...";
  const { major, minor, patch } = version;
  if (major === 0 && minor === 0 && patch === 0) return "latest";
  return `v${major}.${minor}.${patch}`;
};

export const fmt_duration = (start_ts: number, end_ts: number) => {
  const start = new Date(start_ts);
  const end = new Date(end_ts);
  const durr = end.getTime() - start.getTime();
  const seconds = durr / 1000;
  const minutes = Math.floor(seconds / 60);
  const remaining_seconds = seconds % 60;
  return `${
    minutes > 0 ? `${minutes} minute${minutes > 1 ? "s" : ""} ` : ""
  }${remaining_seconds.toFixed(minutes > 0 ? 0 : 1)} seconds`;
};

export function env_to_text(envVars: Types.EnvironmentVar[] | undefined) {
  return envVars?.reduce(
    (prev, { variable, value }) =>
      prev + (prev ? "\n" : "") + `${variable}=${value}`,
    ""
  );
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

/// list_all_items => List All Items
export function snake_case_to_upper_space_case(snake: string) {
  return snake
    .split("_")
    .map((item) => item[0].toUpperCase() + item.slice(1))
    .join(" ");
}
