import { Deployment } from "@monitor/types";
import mongoose from "mongoose";
import { model, Schema } from "mongoose";

export default function deploymentModel() {
	const Conversion = new Schema({
    local: String,
    container: String,
  });

  const EnvironmentVar = new Schema({
    variable: String,
    value: String,
  });

  const schema = new Schema<Deployment>({
    name: { type: String, unique: true, index: true },
    containerName: { type: String, unique: true, index: true }, // for auto pull of frontend repo as well
    owners: [String],
    serverID: { type: String, index: true },
    buildID: { type: String, index: true }, // if deploying a monitor build
    /* to create docker run command */
    image: String, // used if deploying an external image (from docker hub)
    latest: Boolean, // if custom image, use this to add :latest
    ports: [Conversion],
    volumes: [Conversion],
    environment: [EnvironmentVar],
    network: String,
    restart: String,
    postImage: String, // interpolated into run command after the image String
    containerUser: String, // after -u in the run command
    /* to manage repo for static frontend, mounted as a volume */
    repo: String,
    branch: { type: String, default: "master" },
    accessToken: String,
    containerMount: String, // the file path to mount repo on inside the container
  });

	return model("Deployment", schema)
}

export async function getCoreDeployment({ name, mongoUrl }: { name: string; mongoUrl: string }) {
  await mongoose.connect(mongoUrl);

  const deployments = deploymentModel();

  return (await deployments.findOne({ name }).lean().exec()) as
    | Deployment
    | undefined;
}
