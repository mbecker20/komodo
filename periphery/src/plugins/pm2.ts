import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import axios from "axios";
import { PM2_CLIENT_PORT } from "../config";

declare module "fastify" {
	interface FastifyInstance {
		pm2Enabled: {
			get: () => boolean;
			set: (enabled: boolean) => void;
		};
	}
}

const pm2 = fp((app: FastifyInstance, _: {}, done: () => void) => {
	let pm2Enabled = true;
	app.decorate("pm2Enabled", {
		get: () => pm2Enabled,
		set: (enabled: boolean) => {
			pm2Enabled = enabled;
		}
	});
	checkEnabled(app);
	done();
});

async function checkEnabled(app: FastifyInstance) {
	try {
		await axios.get(`http://127.0.0.1:${PM2_CLIENT_PORT}/enabled`);
		app.pm2Enabled.set(true);
	} catch {
		app.pm2Enabled.set(false);
	}
}

export default pm2;