import axios from "axios";
import {
  ReadResponses,
  WriteResponses,
  ExecuteResponses,
  AuthResponses,
} from "./responses";
import {
  ReadRequest,
  WriteRequest,
  ExecuteRequest,
  AuthRequest,
} from "./types";

export type LoginOptions = {
  jwt?: string;
  username?: string;
  password?: string;
  secret?: string;
};

export function MonitorClient(base_url: string, token?: string) {
  let jwt = token ?? "";

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
        return res.jwt;
      } else if (options.secret) {
        const res = await auth({
          type: "LoginWithSecret",
          params: { username: options.username, secret: options.secret },
        });
        jwt = res.jwt;
        return res.jwt;
      }
    }
  };

  const read = async <R extends ReadRequest>(
    request: R
  ): Promise<ReadResponses[R["type"]]> => {
    return await axios({
      method: "post",
      url: base_url + "/read",
      headers: {
        Authorization: `Bearer ${jwt}`,
      },
      data: request,
    }).then(({ data }) => data);
  };

  const write = async <R extends WriteRequest>(
    request: R
  ): Promise<WriteResponses[R["type"]]> => {
    return await axios({
      method: "post",
      url: base_url + "/write",
      headers: {
        Authorization: `Bearer ${jwt}`,
      },
      data: request,
    }).then(({ data }) => data);
  };

  const execute = async <R extends ExecuteRequest>(
    request: R
  ): Promise<ExecuteResponses[R["type"]]> => {
    return await axios({
      method: "post",
      url: base_url + "/execute",
      headers: {
        Authorization: `Bearer ${jwt}`,
      },
      data: request,
    }).then(({ data }) => data);
  };

  return {
    auth,
    login,
    read,
    write,
    execute,
  };
}
