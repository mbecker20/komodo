import { Update } from "@monitor/types";
import { model, Schema } from "mongoose";

export default function updateModel() {
	const Log = new Schema({
    stdout: String,
    stderr: String,
  });

  const schema = new Schema<Update>({
    buildID: { type: String, index: true },
    deploymentID: { type: String, index: true },
    serverID: { type: String, index: true },
    operation: { type: String, index: true },
    command: String,
    log: Log,
    timestamp: Number,
    note: String,
    isError: Boolean,
    operator: { type: String, index: true }, // the userID or username
  });

  return model("Update", schema);
}
