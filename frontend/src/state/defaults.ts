import { Deployment } from "@monitor/types";

export function defaultDeployment(name: string, serverID: string, username: string): Deployment {
	return {
		name,
		serverID,
		owners: [username],
	}
}