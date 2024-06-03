import { Types } from "@monitor/client";

export const fmt_date = (d: Date) => {
  const hours = d.getHours();
  const minutes = d.getMinutes();
  return `${d.getDate()}/${d.getMonth() + 1} @ ${
    hours > 9 ? hours : "0" + hours
  }:${minutes > 9 ? minutes : "0" + minutes}`;
};

export const fmt_date_with_minutes = (d: Date) => {
  // return `${d.toLocaleDateString()} ${d.toLocaleTimeString()}`;
  return d.toLocaleString();
};

export const fmt_version = (version: Types.Version | undefined) => {
  if (!version) return "...";
  const { major, minor, patch } = version;
  if (major === 0 && minor === 0 && patch === 0) return "Latest";
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

export const fmt_operation = (operation: Types.Operation) => {
  return operation.match(/[A-Z][a-z]+|[0-9]+/g)?.join(" ")!;
};

/// list_all_items => List All Items
export function snake_case_to_upper_space_case(snake: string) {
  if (snake.length === 0) return "";
  return snake
    .split("_")
    .map((item) => item[0].toUpperCase() + item.slice(1))
    .join(" ");
}
