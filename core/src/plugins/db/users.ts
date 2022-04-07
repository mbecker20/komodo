import { User } from "@monitor/types";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { Schema } from "mongoose";
import model from "../../util/model";

const users = fp((app: FastifyInstance, _: {}, done: () => void) => {
	const schema = new Schema<User>({
    username: { type: String, index: true, required: true },
    permissions: { type: Number, default: 0 }, 
    password: String,
    avatar: String,
    githubID: { type: Number, index: true },
    enabled: { type: Boolean, default: false },
  });

	app.decorate("users", model(app, "User", schema));

	done();
});

export default users;