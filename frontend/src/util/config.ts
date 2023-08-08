import { Types } from "@monitor/client";

export const RESOURCE_TYPES: Exclude<Types.ResourceTarget["type"], "System">[] =
  ["Alerter", "Build", "Builder", "Deployment", "Repo", "Server"];
