import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { HOST } from "../config";

const listenerURL = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.get("/listenerURL", { onRequest: [app.auth] }, async (req, res) => {
    const { buildID, deploymentID } = req.query as {
      buildID?: string;
      deploymentID?: string;
    };
    if (buildID) {
      res.send(`${HOST}/api/listener/build/${buildID}`);
    } else if (deploymentID) {
      res.send(`${HOST}/api/listener/deployment/${deploymentID}`);
    } else {
      res.status(400);
      res.send();
    }
  });
  done();
});

export default listenerURL;
