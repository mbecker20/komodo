import axios from "axios";
import { ReadResponses, AuthResponses } from "./responses";
import { ReadRequest, WriteRequest, ExecuteRequest, AuthRequest } from "./types";

export type LoginOptions = {
  jwt?: string;
  username?: string;
  password?: string;
  secret?: string;
};

export async function MonitorClient(base_url: string) {
  let jwt = "";

  const auth = async <R extends AuthRequest>(
    request: R
  ): Promise<AuthResponses[R["type"]]> => {
    return await axios({
      method: "post",
      url: base_url + "/auth",
      data: request,
    }).then(({ data }) => data);
  };

  const login = async (options: LoginOptions) => {
    if (options.username) {
      if (options.password) {
        const res = await auth({
          type: "LoginLocalUser",
          params: { username: options.username, password: options.password },
        });
        jwt = res.jwt;
      } else if (options.secret) {
        const res = await auth({
          type: "LoginWithSecret",
          params: { username: options.username, secret: options.secret },
        });
        jwt = res.jwt;
      }
    }
  }

  // const read = async <R extends ReadRequest>(
  //   request: R
  // ): Promise<ReadResponses[R["type"]]> => {
  //   return await axios({
  //     method: "post",
  //     url: base_url + "/read",
  //     headers: {
  //       Authorization: `Bearer ${jwt}`
  //     },
  //     data: request,
  //   }).then(({ data }) => data);
  // };

  return {
    auth,
    login,
    // read,
  };
}
