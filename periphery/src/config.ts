import { getBooleanFromEnv, getNumberFromEnv, getStringFromEnv } from "@monitor/util";

export const LOG = getBooleanFromEnv("LOG", false);
export const PORT = getNumberFromEnv("PORT", 7000);
export const REGISTRY_URL = getStringFromEnv("REGISTRY_URL", "http://localhost:5000/");
export const PASSKEY = getStringFromEnv("PASSKEY", "nfpaowe8hcncew30942j");
export const SYSROOT = getStringFromEnv("SYSROOT", "/home/ubuntu/");
export const ROOT = "/rootDir/";
export const CONTAINER_REPO_ROOT = ROOT + "repos/";
export const SYS_REPO_ROOT = SYSROOT + "repos/";