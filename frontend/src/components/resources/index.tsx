import { RequiredResourceComponents, UsableResource } from "@types";
import { Alerter } from "./alerter";
import { BuildComponents } from "./build";
import { Builder } from "./builder";
import { Deployment } from "./deployment";
import { Repo } from "./repo";
import { ServerComponents } from "./server";
import { Procedure } from "./procedure";

export const ResourceComponents: {
  [key in UsableResource]: RequiredResourceComponents;
} = {
  Alerter,
  Build: BuildComponents,
  Builder,
  Deployment,
  Repo,
  Server: ServerComponents,
  Procedure,
};
