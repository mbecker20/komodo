import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import axios from "axios";
import { PM2_CLIENT_PORT } from "../config";

const pm2 = fp((app: FastifyInstance, _: {}, done: () => void) => {
	app.get("/pm2/processes", { onRequest: [app.auth] }, async (_, res) => {
		try {
			const processes = await getPm2Processes();
			res.send(processes);
		} catch {
			res.status(503);
			res.send("could not reach pm2 client");
		}
	});

	app.get("/pm2/log/:name", { onRequest: [app.auth] }, async (req, res) => {
		const { name } = req.params as { name: string }
		if (name) {
			try {
				const cle = await getPm2Log(name);
				res.send(cle);
			} catch {
				res.status(503);
				res.send("could not reach pm2 client");
			}
		} else {
			res.status(400);
			res.send("no name specified");
		}
	});

	app.get("/pm2/start/:name", { onRequest: [app.auth] }, async (req, res) => {
		const { name } = req.params as { name: string };
		if (name) {
			try {
				const cle = await startPm2(name);
				res.send(cle);
			} catch {
				res.status(503);
				res.send("could not reach pm2 client");
			}
		} else {
			res.status(400);
			res.send("no name specified");
		}
	});

	app.get("/pm2/stop/:name", { onRequest: [app.auth] }, async (req, res) => {
		const { name } = req.params as { name: string };
		if (name) {
			try {
				const cle = await stopPm2(name);
				res.send(cle);
			} catch {
				res.status(503);
				res.send("could not reach pm2 client");
			}
		} else {
			res.status(400);
			res.send("no name specified");
		}
	});

	app.get("/pm2/restart/:name", { onRequest: [app.auth] }, async (req, res) => {
		const { name } = req.params as { name: string };
		if (name) {
			try {
				const cle = await restartPm2(name);
				res.send(cle);
			} catch {
				res.status(503);
				res.send("could not reach pm2 client");
			}
		} else {
			res.status(400);
			res.send("no name specified");
		}
	});

	app.get("/pm2/delete/:name", { onRequest: [app.auth] }, async (req, res) => {
		const { name } = req.params as { name: string };
		if (name) {
			try {
				const cle = await deletePm2(name);
				res.send(cle);
			} catch {
				res.status(503);
				res.send("could not reach pm2 client");
			}
		} else {
			res.status(400);
			res.send("no name specified");
		}
	});

	done();
});

export default pm2;

async function getPm2Processes() {
	return await axios.get(`http://host.docker.internal:${PM2_CLIENT_PORT}/processes`)
		.then(({ data }) => data);
}

async function getPm2Log(name: string) {
	return await axios.get(`http://host.docker.internal:${PM2_CLIENT_PORT}/log/${name}`)
		.then(({ data }) => data);
}

async function startPm2(name: string) {
	return await axios.get(`http://host.docker.internal:${PM2_CLIENT_PORT}/start/${name}`)
		.then(({ data }) => data);
}

async function stopPm2(name: string) {
	return await axios.get(`http://host.docker.internal:${PM2_CLIENT_PORT}/stop/${name}`)
		.then(({ data }) => data);
}

async function restartPm2(name: string) {
	return await axios.get(`http://host.docker.internal:${PM2_CLIENT_PORT}/restart/${name}`)
		.then(({ data }) => data);
}

async function deletePm2(name: string) {
	return await axios.get(`http://host.docker.internal:${PM2_CLIENT_PORT}/delete/${name}`)
		.then(({ data }) => data);
}