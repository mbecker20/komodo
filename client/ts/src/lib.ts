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

type InitOptions =
  | { type: "jwt"; params: { jwt: string } }
  | { type: "api-key"; params: { api_key: string; secret: string } };

export function MonitorClient(url: string, options: InitOptions) {
  const state = {
    jwt: options.type === "jwt" ? options.params.jwt : undefined,
    api_key: options.type === "api-key" ? options.params.api_key : undefined,
    secret: options.type === "api-key" ? options.params.secret : undefined,
  };

  const request = async <Req, Res>(path: string, request: Req) =>
    await axios
      .post<Res>(url + path, request, {
        headers: {
          Authorization: state.jwt,
          "X-API-KEY": state.api_key,
          "X-API-SECRET": state.secret,
        },
      })
      .then(({ data }) => data);

  const auth = async <Req extends AuthRequest>(req: Req) =>
    await request<Req, AuthResponses[Req["type"]]>("/auth", req);

  const read = async <Req extends ReadRequest>(req: Req) =>
    await request<Req, ReadResponses[Req["type"]]>("/read", req);

  const write = async <Req extends WriteRequest>(req: Req) =>
    await request<Req, WriteResponses[Req["type"]]>("/write", req);

  const execute = async <Req extends ExecuteRequest>(req: Req) =>
    await request<Req, ExecuteResponses[Req["type"]]>("/execute", req);

  return { request, auth, read, write, execute };
}
