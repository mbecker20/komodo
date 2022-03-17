import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { getBuildGithubListenerURL, getDeploymentGithubListenerURL } from "../util/helpers";

const listenerURL = fp((app: FastifyInstance, _: {}, done: () => void) => {
	app.get("/listenerURL", { onRequest: [app.auth] }, async (req, res) => {
		const { buildID, deploymentID } = req.query as { buildID?: string; deploymentID?: string }
		if (buildID) {
			const url = await getBuildGithubListenerURL(app, buildID);
			res.send(url);
		} else if (deploymentID) {
			const url = await getDeploymentGithubListenerURL(app, deploymentID);
			res.send(url);
		} else {
			res.status(400);
			res.send();
		}
	});
	done();
});

export default listenerURL;