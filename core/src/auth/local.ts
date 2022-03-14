import { User } from "@monitor/types";
import { compare, hash } from "bcrypt";
import { FastifyInstance, FastifyReply, FastifyRequest } from "fastify";
import fp from "fastify-plugin";
import { PASSWORD_SALT_ROUNDS, TOKEN_EXPIRES_IN } from "../config";

const local = fp((app: FastifyInstance, _: {}, done: () => void) => {
  app.post(
    "/signup",
    { preValidation: [validateSigninInfo] },
    async (req, res) => {
      // returns a jwt as body
      const username = (req.body as User).username;
      const password = (req.body as User).password!;
      const hashedPass = await hash(password, PASSWORD_SALT_ROUNDS);
      try {
        const user = await app.users.create({
          username,
          password: hashedPass,
        });
        const jwt = app.jwt.sign(
          { id: user._id.toString() },
          { expiresIn: TOKEN_EXPIRES_IN }
        );
        res.send(jwt);
      } catch (error) {
        res.status(400);
        res.send("user could not be created");
      }
    }
  );

  app.post(
    "/login/local",
    { preValidation: [validateSigninInfo] },
    async (req, res) => {
      // returns a jwt for user in the body
      const username = (req.body as User).username;
      const password = (req.body as User).password!;
      const user = await app.users.findOne({ username });
      if (user && user.password) {
        try {
          const result = await compare(password, user.password);
          if (result) {
            const jwt = app.jwt.sign(
              { id: user._id.toString() },
              { expiresIn: 3000 }
            );
            res.send(jwt);
          } else {
            res.status(403);
            res.send("credentials could not be authenticated");
          }
        } catch (error) {
          res.status(403);
          res.send("credentials could not be authenticated");
        }
      } else {
        res.status(403);
        res.send("credentials could not be authenticated");
      }
    }
  );

  done();
});

const validateSigninInfo = async (req: FastifyRequest, res: FastifyReply) => {
  const username = (req.body as User).username;
  const password = (req.body as User).password;
  if (!username || !password) {
    res.status(400);
    res.send("Request body must contain username and password");
  }
};

export default local;
