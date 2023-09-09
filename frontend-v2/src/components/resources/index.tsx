import { RequiredResourceComponents, UsableResource } from "@types";
import { Deployment } from "./deployment";
import { Server } from "./server";
import { Alerter } from "./alerter";
import { Build } from "./build";
import { Builder } from "./builder";
import { Repo } from "./repo";

export const ResourceComponents: {
  [key in UsableResource]: RequiredResourceComponents;
} = {
  Alerter,
  Build,
  Builder,
  Deployment,
  Server,
  Repo,
};
