import { CoreSecrets } from "@monitor/types";
import {
  getBooleanFromEnv,
  getNumberFromEnv,
  getStringFromEnv,
  readJSONFile,
} from "@monitor/util-node";
import { join } from "path";

export const CORE_SERVER_NAME = getStringFromEnv(
  "CORE_SERVER_NAME",
  "Monitor Core"
);
export let SECRETS: CoreSecrets = readJSONFile("/secrets/secrets.json");
export function refreshSecrets() {
  SECRETS = readJSONFile("/secrets/secrets.json");
}
export const LOGGER = getBooleanFromEnv("LOGGER", false);
export const PORT = getNumberFromEnv("PORT", 9000);
export const HOST = getStringFromEnv("HOST", "http://localhost:" + PORT);
export const MONGO_URL = getStringFromEnv(
  "MONGO_URL",
  "mongodb://127.0.0.1:27017/monitor"
);
export const TOKEN_EXPIRES_IN = getStringFromEnv("TOKEN_EXPIRES_IN", "7d");
export const INVITE_TOKEN_EXPIRES_IN = getNumberFromEnv(
  "INVITE_TOKEN_EXPIRES_IN",
  2.592e8 // 3 days
);
export const PASSWORD_SALT_ROUNDS = getNumberFromEnv("PASSWORD_SALT_ROUNDS", 8);
export const SYSROOT = getStringFromEnv("SYSROOT", "/home/ubuntu/"); // the root folder monitor has access to, prepends volumes mounted using useSysroot
export const ROOT = "/monitor-root/"; // the root folder in the container that SYSROOT is mounted on
export const DEPLOYDATA_ROOT = "deployments/";
export const BUILD_REPO_PATH = join(ROOT, "builds");
export const DEPLOYMENT_REPO_PATH = join(ROOT, "repos");
export const SYS_DEPLOYMENT_REPO_PATH = join(SYSROOT, "repos");
// export const REGISTRY_URL = getStringFromEnv("REGISTRY_URL", "localhost:5000/");
export const FRONTEND_PATH = getStringFromEnv("FRONTEND_PATH", "/frontend");
export const SYSTEM_OPERATOR = "Monitor";
export const PERMISSIONS_DENY_LOG = {
  stderr: "Someone tried to access this route without appropriate permissions",
};
export const UPDATES_PER_REQUEST = getNumberFromEnv("UPDATES_PER_REQUEST", 10);
export const SERVER_CHECK_TIMEOUT = getNumberFromEnv(
  "SERVER_CHECK_TIMEOUT",
  1000
);

export const SERVER_STATS_INTERVAL = getNumberFromEnv("SERVER_STATS_INTERVAL_MINUTES", 5) * 60 * 1000; // 5 minute check default
export const CLEAR_ALREADY_ALERTED_INTERVAL = getNumberFromEnv("CLEAR_ALREADY_ALERTED_INTERVAL_HOUR", 24) * 60 * 60 * 1000; // 24 hour default clear interval
export const SLACK_CHANNEL = getStringFromEnv("SLACK_CHANNEL", "");
export const CPU_USAGE_NOTIFY_LIMIT = getNumberFromEnv("CPU_USAGE_NOTIFY_LIMIT", 50);
export const MEM_USAGE_NOTIFY_LIMIT = getNumberFromEnv("MEM_USAGE_NOTIFY_LIMIT", 75);
export const DISK_USAGE_NOTIFY_LIMIT = getNumberFromEnv("DISK_USAGE_NOTIFY_LIMIT", 75);