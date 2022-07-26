import { USER_UPDATE } from "@monitor/util";
import axios from "axios";
import { FastifyInstance } from "fastify";
import fastifyOauth2, { OAuth2Namespace } from "fastify-oauth2";
import fp from "fastify-plugin";
import { HOST, SECRETS, TOKEN_EXPIRES_IN } from "../../config";

declare module "fastify" {
  interface FastifyInstance {
    github: OAuth2Namespace;
  }
}

const github = fp((app: FastifyInstance, _: {}, done: () => void) => {
	app.register(fastifyOauth2, {
    name: "github",
    scope: [], // empty for only basic access to acct, ie info that is is already public about your acct.
    credentials: {
      client: {
        id: SECRETS.GITHUB_OAUTH.ID,
        secret: SECRETS.GITHUB_OAUTH.SECRET,
      },
      auth: fastifyOauth2.GITHUB_CONFIGURATION,
    },
    // location.replace to this url to log in
    startRedirectPath: "/login/github",
    // github redirects here after user logs in
    callbackUri: `${HOST}/login/github/callback`,
  });

  app.get("/login/github/callback", async (req, res) => {
    const token = await app.github.getAccessTokenFromAuthorizationCodeFlow(req);
    const profile = await getGithubProfile(token.access_token);
    const existingUser = await app.users.findOne({
      githubID: profile.githubID,
    });
    if (existingUser) {
      const jwt = app.jwt.sign(
        { id: existingUser._id!.toString() },
        { expiresIn: TOKEN_EXPIRES_IN }
      );
      res.redirect(
        `${HOST}/?token=${jwt}`
      );
    } else {
      const createdUser = await app.users.create(profile);
      app.broadcast(USER_UPDATE, {}, app.adminUserFilter);
      const jwt = app.jwt.sign(
        { id: createdUser._id!.toString() },
        { expiresIn: TOKEN_EXPIRES_IN }
      );
      res.redirect(
        `${HOST}/?token=${jwt}`
      );
    }
  });

	done();
});

async function getGithubProfile(token: string) {
  const profile = await axios
    .get("https://api.github.com/user", {
      headers: {
        Authorization: `token ${token}`,
      },
    })
    .then(({ data }) => data);
  return {
    username: profile.login as string,
    githubID: profile.id as number,
    avatar: profile.avatar_url as string,
  };
}

export default github;