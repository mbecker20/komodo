import { User } from "@monitor/types";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { Schema } from "mongoose";

const users = fp((app: FastifyInstance, _: {}, done: () => void) => {
	const schema = new Schema<User>({
    username: { type: String, index: true, required: true },
    permissions: { type: Number, default: 0 }, 
    password: String,
    avatar: String,
    githubID: { type: Number, index: true },
  });

	app.decorate("users", app.mongoose.model("User", schema));

	done();
});

export default users;