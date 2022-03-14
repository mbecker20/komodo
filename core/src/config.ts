import {
  getBooleanFromEnv,
  getNumberFromEnv,
  getStringFromEnv,
  readJSONFile,
} from "@monitor/util";

export const SECRETS = readJSONFile("secrets.json");
export const LOG = getBooleanFromEnv("LOG", false);
export const PORT = getNumberFromEnv("PORT", 8000);
export const HOST = getStringFromEnv("HOST", "http://localhost:" + PORT);
export const MONGO_URL = getStringFromEnv(
  "MONGO_URL",
  "mongodb://localhost:27017/monitor"
);
export const TOKEN_EXPIRES_IN = getNumberFromEnv("TOKEN_EXPIRES_IN", 3000);
export const PASSWORD_SALT_ROUNDS = getNumberFromEnv("PASSWORD_SALT_ROUNDS", 8);
