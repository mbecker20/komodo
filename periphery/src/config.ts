import { getBooleanFromEnv, getNumberFromEnv, getStringFromEnv } from "@monitor/util";

export const LOG = getBooleanFromEnv("LOG", false);
export const PORT = getNumberFromEnv("PORT", 7000);
export const PASSKEY = getStringFromEnv("PASSKEY", "nfpaowe8hcncew30942j");