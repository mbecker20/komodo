import { EnvironmentVar } from "@monitor/types";
import { readFileSync } from "fs-extra";

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

export function readJSONFile<T = any>(path: string): T {
  const raw = readFileSync(path);
  return JSON.parse(raw as any);
}

export function parseDotEnvToEnvVars(dotenv: string) {
  return dotenv
    .split("\n")
    .filter(entry => entry.length > 0)
    .map(entry => entry.replaceAll("\"", "").split("="))
    .map(([variable, value]) => ({ variable, value }));
}

export function environmentIncludes(newVar: string, environment: EnvironmentVar[]) {
  for (const { variable } of environment) {
    if (newVar === variable) {
      return true;
    }
  }
  return false;
}