import { Types } from "@komodo/client";

export const fmt_date = (d: Date) => {
  const hours = d.getHours();
  const minutes = d.getMinutes();
  return `${fmt_month(d.getMonth())} ${d.getDate()} ${
    hours > 9 ? hours : "0" + hours
  }:${minutes > 9 ? minutes : "0" + minutes}`;
};

export const fmt_utc_date = (d: Date) => {
  const hours = d.getUTCHours();
  const minutes = d.getUTCMinutes();
  return `${fmt_month(d.getUTCMonth())} ${d.getUTCDate()} ${
    hours > 9 ? hours : "0" + hours
  }:${minutes > 9 ? minutes : "0" + minutes}`;
};

const fmt_month = (month: number) => {
  switch (month) {
    case 0:
      return "Jan";
    case 1:
      return "Feb";
    case 2:
      return "Mar";
    case 3:
      return "Apr";
    case 4:
      return "May";
    case 5:
      return "Jun";
    case 6:
      return "Jul";
    case 7:
      return "Aug";
    case 8:
      return "Sep";
    case 9:
      return "Oct";
    case 10:
      return "Nov";
    case 11:
      return "Dec";
  }
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

export const fmt_upper_camelcase = (input: string) => {
  return input.match(/[A-Z][a-z]+|[0-9]+/g)?.join(" ")!;
};

/// list_all_items => List All Items
export function snake_case_to_upper_space_case(snake: string) {
  if (snake.length === 0) return "";
  return snake
    .split("_")
    .map((item) => item[0].toUpperCase() + item.slice(1))
    .join(" ");
}

const BYTES_PER_MB = 1e6;
const BYTES_PER_GB = BYTES_PER_MB * 1000;

export function format_size_bytes(size_bytes: number) {
  if (size_bytes > BYTES_PER_GB) {
    return `${(size_bytes / BYTES_PER_GB).toFixed(1)} GB`;
  } else {
    return `${(size_bytes / BYTES_PER_MB).toFixed(1)} MB`;
  }
}
