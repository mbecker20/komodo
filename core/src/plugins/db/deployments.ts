import { Deployment } from "@monitor/types";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { Schema } from "mongoose";
import model from "../../util/model";
import { Conversion, EnvironmentVar, Volume } from "./misc";

const deployments = fp((app: FastifyInstance, _: {}, done: () => void) => {
	const schema = new Schema<Deployment>({
    name: { type: String, unique: true, index: true },
    containerName: { type: String, unique: true, index: true }, // for auto pull of frontend repo as well
    owner: { type: String, index: true },
    serverID: { type: String, index: true },
    buildID: { type: String, index: true }, // if deploying a monitor build
    /* to create docker run command */
    image: String, // used if deploying an external image (from docker hub)
    latest: Boolean, // if custom image, use this to add :latest
    ports: [Conversion],
    volumes: [Volume],
    environment: [EnvironmentVar],
    network: String,
    restart: String,
    postImage: String, // interpolated into run command after the image String
    containerUser: String, // after -u in the run command
    /* to manage repo for static frontend, mounted as a volume */
    repo: String,
    branch: String,
    accessToken: String,
    containerMount: String, // the file path to mount repo on inside the container
  });

	app.decorate("deployments", model(app, "Deployment", schema));

	done();
});

export default deployments;