import axios from "axios";
import {
  AuthResponses,
  ExecuteResponses,
  ReadResponses,
  UserResponses,
  WriteResponses,
} from "./responses.js";
import {
  AuthRequest,
  ExecuteRequest,
  ReadRequest,
  UserRequest,
  WriteRequest,
} from "./types.js";

export * as Types from "./types.js";

type InitOptions =
  | { type: "jwt"; params: { jwt: string } }
  | { type: "api-key"; params: { key: string; secret: string } };

export function KomodoClient(url: string, options: InitOptions) {
  const state = {
    jwt: options.type === "jwt" ? options.params.jwt : undefined,
    key: options.type === "api-key" ? options.params.key : undefined,
    secret: options.type === "api-key" ? options.params.secret : undefined,
  };

  const request = async <Req, Res>(path: string, request: Req) =>
    await axios
      .post<Res>(url + path, request, {
        headers: {
          Authorization: state.jwt,
          "X-API-KEY": state.key,
          "X-API-SECRET": state.secret,
        },
      })
      .then(({ data }) => data);

  const auth = async <
    T extends AuthRequest["type"],
    Req extends Extract<AuthRequest, { type: T }>
  >(
    type: T,
    params: Req["params"]
  ) =>
    await request<
      { type: T; params: Req["params"] },
      AuthResponses[Req["type"]]
    >("/auth", {
      type,
      params,
    });

  const user = async <
    T extends UserRequest["type"],
    Req extends Extract<UserRequest, { type: T }>
  >(
    type: T,
    params: Req["params"]
  ) =>
    await request<
      { type: T; params: Req["params"] },
      UserResponses[Req["type"]]
    >("/user", { type, params });

  const read = async <
    T extends ReadRequest["type"],
    Req extends Extract<ReadRequest, { type: T }>
  >(
    type: T,
    params: Req["params"]
  ) =>
    await request<
      { type: T; params: Req["params"] },
      ReadResponses[Req["type"]]
    >("/read", { type, params });

  const write = async <
    T extends WriteRequest["type"],
    Req extends Extract<WriteRequest, { type: T }>,
  >(
    type: T,
    params: Req["params"]
  ) =>
    await request<
      { type: T; params: Req["params"] },
      WriteResponses[Req["type"]]
    >("/write", { type, params });

  const execute = async <
    T extends ExecuteRequest["type"],
    Req extends Extract<ExecuteRequest, { type: T }>
  >(
    type: T,
    params: Req["params"]
  ) =>
    await request<
      { type: T; params: Req["params"] },
      ExecuteResponses[Req["type"]]
    >("/execute", { type, params });

  return { request, auth, user, read, write, execute };
}
