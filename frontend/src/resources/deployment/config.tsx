import { Config } from "@components/config/Config"
import { useRead } from "@hooks";
import { Types } from "@monitor/client";
import { useState } from "react"
import { useParams } from "react-router-dom";

export const DeploymentConfig = () => {
	const id = useParams().deploymentId;
	const deployment = useRead("GetDeployment", { id });
	const [update, set] = useState<Partial<Types.DeploymentConfig>>({});
	if (deployment.data?.config) {
		return <Config config={deployment.data?.config as any} update={update} set={set} />;
	} else {
		// loading
		return null
	}
}