import { getBooleanFromEnv, getNumberFromEnv } from "@monitor/util";

export const LOG = getBooleanFromEnv("LOG", false);
export const PORT = getNumberFromEnv("PORT", 8000);