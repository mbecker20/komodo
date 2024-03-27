import { RequiredResourceComponents, UsableResource } from "@types";
import { AlerterComponents } from "./alerter";
import { BuildComponents } from "./build";
import { BuilderComponents } from "./builder";
import { DeploymentComponents } from "./deployment";
import { RepoComponents } from "./repo";
import { ServerComponents } from "./server";
import { ProcedureComponents } from "./procedure/index";

export const ResourceComponents: {
  [key in UsableResource]: RequiredResourceComponents;
} = {
  Alerter: AlerterComponents,
  Build: BuildComponents,
  Builder: BuilderComponents,
  Deployment: DeploymentComponents,
  Repo: RepoComponents,
  Server: ServerComponents,
  Procedure: ProcedureComponents,
};
