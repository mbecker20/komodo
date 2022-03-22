import { Collection, Log } from "@monitor/types";
import { readFileSync } from "fs-extra";

export function readJSONFile<T = any>(path: string): T {
  const raw = readFileSync(path);
  return JSON.parse(raw as any);
}

export function getStringFromEnv(varName: string, defaultValue: string) {
  return process.env[varName] ? process.env[varName]! : defaultValue;
}

export function getNumberFromEnv(varName: string, defaultValue: number) {
  return process.env[varName] ? Number(process.env[varName]!) : defaultValue;
}

export function getBooleanFromEnv(varName: string, defaultValue: boolean) {
  const variable = process.env[varName];
  if (variable === "true") return true;
  else if (variable === "false") return false;
  else return defaultValue;
}

export function objFrom2Arrays<T>(keys: string[], entries: T[]): Collection<T | undefined> {
  return Object.fromEntries(
    keys.map((id, index) => {
      return [id, entries[index]];
    })
  );
}

export function filterOutFromObj<T>(
  obj: T,
  idsToFilterOut: string[]
) {
  return Object.fromEntries(
    Object.entries(obj).filter((entry) => {
      return !idsToFilterOut.includes(entry[0]);
    })
  );
}

export function timestamp() {
  return Math.floor(Date.now() / 1000);
}

export function combineLogs(log0: Log, log1: Log): Log {
  return {
    stdout:
      (log0.stdout ? log0.stdout : "") +
      (log0.stdout && log1.stdout ? ", " : "") +
      (log1.stdout ? log1.stdout : ""),
    stderr:
      (log0.stderr ? log0.stderr : "") +
      (log0.stderr && log1.stderr ? ", " : "") +
      (log1.stderr ? log1.stderr : ""),
  };
}
