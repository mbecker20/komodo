import { Log, PM2Process, Server } from "@monitor/types";
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

export async function getPeripheryPm2Log({ address, passkey }: Server, name: string) {
	try {
		return await axios
			.get<Log>(`${address}/pm2/log/${name}`, {
				headers: {
					Authorization: passkey || SECRETS.PASSKEY,
				},
			})
			.then(({ data }) => data);
	} catch (error) {
		return { stderr: "could not reach pm2 client" }
	}
}

export async function startPeripheryPm2({ address, passkey }: Server, name: string) {
	try {
		return await axios
			.get<Log>(`${address}/pm2/start/${name}`, {
				headers: {
					Authorization: passkey || SECRETS.PASSKEY,
				},
			})
			.then(({ data }) => data);
	} catch (error) {
		return { stderr: "could not reach pm2 client" }
	}
}

export async function stopPeripheryPm2({ address, passkey }: Server, name: string) {
	try {
		return await axios
			.get<Log>(`${address}/pm2/stop/${name}`, {
				headers: {
					Authorization: passkey || SECRETS.PASSKEY,
				},
			})
			.then(({ data }) => data);
	} catch (error) {
		return { stderr: "could not reach pm2 client" }
	}
}

export async function restartPeripheryPm2({ address, passkey }: Server, name: string) {
	try {
		return await axios
			.get<Log>(`${address}/pm2/restart/${name}`, {
				headers: {
					Authorization: passkey || SECRETS.PASSKEY,
				},
			})
			.then(({ data }) => data);
	} catch (error) {
		return { stderr: "could not reach pm2 client" }
	}
}

export async function deletePeripheryPm2({ address, passkey }: Server, name: string) {
	try {
		return await axios
			.get<Log>(`${address}/pm2/delete/${name}`, {
				headers: {
					Authorization: passkey || SECRETS.PASSKEY,
				},
			})
			.then(({ data }) => data);
	} catch (error) {
		return { stderr: "could not reach pm2 client" }
	}
}