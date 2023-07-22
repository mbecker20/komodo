import axios from "axios";
import {
  AuthResponses,
  ExecuteResponses,
  ReadResponses,
  WriteResponses,
} from "./responses";
import {
  AuthRequest,
  ExecuteRequest,
  ReadRequest,
  WriteRequest,
} from "./types";

export * as Types from "./types";

export type LoginOptions = {
  jwt?: string;
  username?: string;
  password?: string;
  secret?: string;
};

export function MonitorClient(base_url: string, token?: string) {
  const state = { jwt: token ?? "" };

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
        state.jwt = res.jwt;
        return res.jwt;
      } else if (options.secret) {
        const { jwt } = await auth({
          type: "LoginWithSecret",
          params: { username: options.username, secret: options.secret },
        });
        state.jwt = jwt;
        return jwt;
      }
    }
  };

  const request = async <Req, Res>(
    path: string,
    request: Req
  ): Promise<Res> => {
    return await axios({
      method: "post",
      url: base_url + path,
      headers: {
        Authorization: `Bearer ${state.jwt}`,
      },
      data: request,
    }).then(({ data }) => data);
  };

  const read = async <R extends ReadRequest>(
    req: R
  ): Promise<ReadResponses[R["type"]]> => {
    return await request("/read", req);
  };

  const write = async <R extends WriteRequest>(
    req: R
  ): Promise<WriteResponses[R["type"]]> => {
    return await request("/write", req);
  };

  const execute = async <R extends ExecuteRequest>(
    req: R
  ): Promise<ExecuteResponses[R["type"]]> => {
    return await request("/execute", req);
  };

  return {
    auth,
    login,
    read,
    write,
    execute,
  };
}
