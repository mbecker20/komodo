import { AccountAccess } from "@monitor/types";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { Schema } from "mongoose";
import { SECRETS } from "../../config";
import model from "../../util/model";

const accounts = fp((app: FastifyInstance, _: {}, done: () => void) => {
  const schema = new Schema<AccountAccess>({
		type: String,
    username: { type: String, index: true, required: true },
    users: { type: [String], default: [] },
  });

	const accountModel = model(app, "Account", schema);

  app.decorate("accounts", accountModel);

	const githubUsernames = Object.keys(SECRETS.GITHUB_ACCOUNTS);
	const dockerUsernames = Object.keys(SECRETS.DOCKER_ACCOUNTS);

	accountModel.find({}).then((accounts) => {
    const githubAccounts = accounts.filter((act) => act.type === "github");
    const dockerAccounts = accounts.filter((act) => act.type === "docker");
    githubUsernames.forEach((username) => {
      if (githubAccounts.filter((act) => act.username === username).length === 0) {
        // account not added to db
				accountModel.create({ type: "github", username, users: [] });
      }
    });
    dockerUsernames.forEach((username) => {
			if (
        dockerAccounts.filter((act) => act.username === username).length === 0
      ) {
        // account not added to db
        accountModel.create({ type: "docker", username, users: [] });
      }
		});
		// now delete any from db that are no longer specified in secrets
		githubAccounts.forEach(act => {
			if (!githubUsernames.includes(act.username)) {
				accountModel.findByIdAndDelete(act._id!);
			}
		})
		dockerAccounts.forEach(act => {
			if (!dockerUsernames.includes(act.username)) {
        accountModel.findByIdAndDelete(act._id!);
      }
		});
  });

  done();
});

export default accounts;
