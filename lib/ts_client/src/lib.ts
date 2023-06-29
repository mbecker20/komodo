import axios from "axios";
import { ApiResponses, AuthResponses } from "./responses";
import { ApiRequest, AuthRequest } from "./types";

export type ClientOptions = {
  jwt?: string;
  username?: string;
  password?: string;
  secret?: string;
};

export async function MonitorClient(base_url: string, options?: ClientOptions) {
  let jwt = options?.jwt;

  const auth = async <R extends AuthRequest>(type: R["type"], params?: R["params"]): Promise<AuthResponses[R["type"]]> => {
    return await axios({
      method: "post",
      url: base_url + "/auth",
      data: {
        type,
        params: params || {}
      },
    }).then(({ data }) => data);
  }

  if (!jwt) {
    if (options?.username) {
      if (options?.password) {
        const res = await auth("LoginLocalUser", {  });
      } else if (options?.secret) {
      }
    }
  }

  // const api = async <R extends ApiRequest>(
  //   request: R
  // ): Promise<ApiResponses[R["type"]]> => {
  //   return await axios({
  //     method: "post",
  //     url: base_url + "/api",
  //     headers: {
  //       Authorization: `Bearer ${jwt}`
  //     },
  //     data: request,
  //   }).then(({ data }) => data);
  // };

  return {
    auth,

  };
}
