import { client } from "..";
import { Deployment, DeploymentWithContainer } from "../types";
import { generateQuery, QueryObject } from "./helpers";

// deployment

export function list_deployments(query?: QueryObject): Promise<DeploymentWithContainer[]> {
	return client.get(`/api/deployment/list${generateQuery(query)}`)
}

export function get_deployment(id: string): Promise<DeploymentWithContainer> {
	return client.get(`/api/deployment/${id}`);
}

export function create_deployment(name: string, server_id: string): Promise<Deployment> {
	return client.post("/api/deployment/create", { name, server_id });
}

export function create_full_deployment(deployment: Deployment): Promise<Deployment> {
	return client.post("/api/deployment/create_full", deployment)
}

export function delete_deployment<Deployment>(deployment_id: string): Promise<Deployment> {
	return client.delete(`/api/deployment/delete/${deployment_id}`);
}

