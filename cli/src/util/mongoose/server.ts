import { Server } from "@monitor/types";
import { model, Schema } from "mongoose";

export default function serverModel() {
	const schema = new Schema<Server>({
    name: { type: String, unique: true },
    address: String,
    passkey: String,
    enabled: { type: Boolean, default: true },
    isCore: Boolean,
  });

  return model("Server", schema);
}