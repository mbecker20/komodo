import { Build } from "@monitor/types";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { Schema } from "mongoose";
import model from "../../util/model";
import { Command, DockerBuildArgs } from "./misc";

const builds = fp((app: FastifyInstance, _: {}, done: () => void) => {
  const schema = new Schema<Build>({
    name: { type: String, unique: true, index: true },
    pullName: { type: String, unique: true, index: true },
    commands: [Command],
    /* repo related */
    repo: String,
    branch: String,
    accessToken: String,
    onClone: [Command],
    /* build related */
    cliBuild: Command,
    dockerBuildArgs: DockerBuildArgs,
    owners: [String],
  });

  app.decorate("builds", model(app, "Build", schema));

  done();
});

export default builds;
