import { Build } from "@monitor/types";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { Schema } from "mongoose";
import model from "../../util/model";

const builds = fp((app: FastifyInstance, _: {}, done: () => void) => {
  const schema = new Schema<Build>({
    name: { type: String, unique: true, index: true },
    /* repo related */
    repo: String,
    branch: String,
    accessToken: String, // to gain access to private repos
    /* build related */
    buildPath: String, // build folder relative to repo root
    dockerfilePath: String, // relative to buildPath
    imageName: { type: String, index: true }, // derived on build creation
    owners: [String],
  });

  app.decorate("builds", model(app, "Build", schema));

  done();
});

export default builds;
