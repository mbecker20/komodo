import { CORE_IMAGE } from "../../config";
import { Config } from "../../types";
import { execute } from "../execute";

export async function startMonitorCore({
  monitorCore: {
		
	},
  mongo: { url: mongoURL },
  registry: { url: registryURL },
}: Config) {
  const command = `docker run -d --name monitor-core -e MONGO_URL=${mongoURL}${
    registryURL ? ` -e REGISTRY_URL=${registryURL}` : ""
  } ${CORE_IMAGE}`;
	return await execute(command);
}
