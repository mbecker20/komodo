import { Collection } from "@monitor/types";
import { readFileSync } from "fs";

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

export function objFrom2Arrays<T>(keys: string[], entries: T[]): Collection<T> {
  return Object.fromEntries(
    keys.map((id, index) => {
      return [id, entries[index]];
    })
  );
}
