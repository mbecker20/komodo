import { User } from "@monitor/types";
import { Schema, model } from "mongoose";

export default function userModel() {
	const schema = new Schema<User>({
    username: { type: String, index: true, required: true },
    permissions: { type: Number, default: 0 },
    password: String,
    avatar: String,
    githubID: { type: Number, index: true },
  });

	return model("User", schema);
}