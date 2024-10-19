import axios from "axios";
export * as Types from "./types.js";
export function KomodoClient(url, options) {
    const state = {
        jwt: options.type === "jwt" ? options.params.jwt : undefined,
        key: options.type === "api-key" ? options.params.key : undefined,
        secret: options.type === "api-key" ? options.params.secret : undefined,
    };
    const request = async (path, request) => await axios
        .post(url + path, request, {
        headers: {
            Authorization: state.jwt,
            "X-API-KEY": state.key,
            "X-API-SECRET": state.secret,
        },
    })
        .then(({ data }) => data);
    const auth = async (type, params) => await request("/auth", {
        type,
        params,
    });
    const user = async (type, params) => await request("/user", { type, params });
    const read = async (type, params) => await request("/read", { type, params });
    const write = async (type, params) => await request("/write", { type, params });
    const execute = async (type, params) => await request("/execute", { type, params });
    return { request, auth, user, read, write, execute };
}
