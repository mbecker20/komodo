import { Build } from "@monitor/types";
import { REGISTRY_URL, REPO_PATH } from "../../config";
import { toDashedName } from "../helpers";

export function createDockerBuild({
  name,
  buildPath,
  dockerfilePath,
  pullName,
}: Build) {
  return `cd ${REPO_PATH}${pullName}${
    buildPath ? (buildPath[0] === "/" ? buildPath : "/" + buildPath) : ""
  } && docker build -t ${
    REGISTRY_URL + toDashedName(name)
  } -f ${dockerfilePath} . && docker push ${
    REGISTRY_URL + toDashedName(name)
  }`;
}