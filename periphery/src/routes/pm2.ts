import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";

const pm2 = fp((app: FastifyInstance, _: {}, done: () => void) => {
	app.get("/pm2List", { onRequest: [app.auth] }, async (_, res) => {
		// if (app.pm2Enabled.get()) {
		// 	const processes = await listPm2Processes();
		// 	res.send(processes);
		// } else {
		// 	res.send([]);
		// }
	});
	
	done();
});

export default pm2;