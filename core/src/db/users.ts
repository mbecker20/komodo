import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { Schema } from "mongoose";

const users = fp((app: FastifyInstance, _: {}, done: () => void) => {
	const schema = new Schema({
    username: { type: String, index: true },
    password: String,
    avatar: String,
    githubID: { type: Number, index: true },
  });

	app.decorate("users", app.mongoose.model("User", schema));

	done();
});

export default users;