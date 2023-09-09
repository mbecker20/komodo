import { RequiredResourceComponents, UsableResource } from "@types";
import { Alerter } from "./alerter";
import { Build } from "./build";
import { Builder } from "./builder";
import { Deployment } from "./deployment";
import { Repo } from "./repo";
import { Server } from "./server";

export const ResourceComponents: {
  [key in UsableResource]: RequiredResourceComponents;
} = {
  Alerter,
  Build,
  Builder,
  Deployment,
  Repo,
  Server,
};
