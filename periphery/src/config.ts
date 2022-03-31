import { PeripherySecrets } from "@monitor/types";
import { getBooleanFromEnv, getNumberFromEnv, getStringFromEnv, readJSONFile } from "@monitor/util";

export const SECRETS = readJSONFile("/secrets/secrets.json") as PeripherySecrets;
export const LOG = getBooleanFromEnv("LOG", false);
export const PORT = getNumberFromEnv("PORT", 8000);
export const REGISTRY_URL = getStringFromEnv("REGISTRY_URL", "http://localhost:5000/");
export const SYSROOT = getStringFromEnv("SYSROOT", "/home/ubuntu/");
export const ROOT = "/rootDir/";
export const CONTAINER_REPO_ROOT = ROOT + "repos/";
export const SYS_REPO_ROOT = SYSROOT + "repos/";