import { CommandLogError, PM2Process, Server } from "@monitor/types";
import { generateQuery } from "@monitor/util";
import axios from "axios";
import { SECRETS } from "../../config";

export async function getPeripheryPm2Processes({ address, passkey }: Server) {
	try {
		return await axios
			.get<PM2Process[]>(`${address}/pm2/processes`, {
				headers: {
					Authorization: passkey || SECRETS.PASSKEY,
				},
			})
			.then(({ data }) => data);
	} catch (error) {
		return []
	}
}

export async function getPeripheryPm2Log({ address, passkey }: Server, name: string, lines = 50) {
	try {
		return await axios
			.get<CommandLogError>(`${address}/pm2/log/${name}` + generateQuery({ lines }), {
				headers: {
					Authorization: passkey || SECRETS.PASSKEY,
				},
			})
			.then(({ data }) => data);
	} catch (error) {
		return { command: "try to get log", log: { stderr: "could not reach pm2 client" }, isError: true };
	}
}

export async function startPeripheryPm2({ address, passkey }: Server, name: string) {
	try {
		return await axios
			.get<CommandLogError>(`${address}/pm2/start/${name}`, {
				headers: {
					Authorization: passkey || SECRETS.PASSKEY,
				},
			})
			.then(({ data }) => data);
	} catch (error) {
		return { command: "try to start process", log: { stderr: "could not reach pm2 client" }, isError: true };
	}
}

export async function stopPeripheryPm2({ address, passkey }: Server, name: string) {
	try {
		return await axios
			.get<CommandLogError>(`${address}/pm2/stop/${name}`, {
				headers: {
					Authorization: passkey || SECRETS.PASSKEY,
				},
			})
			.then(({ data }) => data);
	} catch (error) {
		return { command: "try to stop process", log: { stderr: "could not reach pm2 client" }, isError: true };
	}
}

export async function restartPeripheryPm2({ address, passkey }: Server, name: string) {
	try {
		return await axios
			.get<CommandLogError>(`${address}/pm2/restart/${name}`, {
				headers: {
					Authorization: passkey || SECRETS.PASSKEY,
				},
			})
			.then(({ data }) => data);
	} catch (error) {
		return { command: "try to restart process", log: { stderr: "could not reach pm2 client" }, isError: true };
	}
}

export async function deletePeripheryPm2({ address, passkey }: Server, name: string) {
	try {
		return await axios
			.get<CommandLogError>(`${address}/pm2/delete/${name}`, {
				headers: {
					Authorization: passkey || SECRETS.PASSKEY,
				},
			})
			.then(({ data }) => data);
	} catch (error) {
		return { command: "try to delete process", log: { stderr: "could not reach pm2 client" }, isError: true };
	}
}

export async function flushPm2Logs({ address, passkey }: Server, name?: string) {
	try {
		return await axios
			.get<CommandLogError>(`${address}/pm2/flush` + generateQuery({ name }), {
				headers: {
					Authorization: passkey || SECRETS.PASSKEY,
				},
			})
			.then(({ data }) => data);
	} catch (error) {
		return { command: "try to flush logs", log: { stderr: "could not reach pm2 client" }, isError: true };
	}
}