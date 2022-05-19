import PM2 from "pm2";
import { PM2Process } from "@monitor/types"

export function listPm2Processes(): Promise<PM2Process[]> {
	return new Promise((res, rej) => {
		PM2.list((err, list) => {
			if (err) {
				rej(err);
			} else {
				res(list.map(p => ({
					pid: p.pid,
					name: p.name,
					status: p.pm2_env?.status,
					cpu: p.monit?.cpu,
					memory: p.monit?.memory,
					uptime: p.pm2_env?.pm_uptime ? Date.now() - p.pm2_env?.pm_uptime : 0,
					createdAt: (p.pm2_env as any)?.created_at,
					restarts: p.pm2_env?.unstable_restarts
				})))
			}
		})
	})
}