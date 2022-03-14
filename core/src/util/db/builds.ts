import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { Schema } from "mongoose";

const builds = fp((app: FastifyInstance, _: {}, done: () => void) => {
  const schema = new Schema({
    name: { type: String, unique: true, index: true },
    /* repo related */
    repo: String,
    branch: String,
    accessToken: String, // to gain access to private repos
    /* build related */
    buildPath: String, // build folder relative to repo root
    dockerfilePath: String, // relative to buildPath
    pullName: { type: String, index: true }, // derived on build creation
    imageName: String, // derived on build creation
    owner: { type: String, index: true }, // userID / username
  });

  app.decorate("builds", app.mongoose.model("Build", schema));

  done();
});

export default builds;
