export * as Types from "./types.js";
/** Initialize a new client for Komodo */
export function KomodoClient(url, options) {
    const state = {
        jwt: options.type === "jwt" ? options.params.jwt : undefined,
        key: options.type === "api-key" ? options.params.key : undefined,
        secret: options.type === "api-key" ? options.params.secret : undefined,
    };
    const request = async (path, request) => new Promise(async (res, rej) => {
        try {
            let response = await fetch(url + path, {
                method: "POST",
                body: JSON.stringify(request),
                headers: {
                    ...(state.jwt
                        ? {
                            authorization: state.jwt,
                        }
                        : state.key && state.secret
                            ? {
                                "x-api-key": state.key,
                                "x-api-secret": state.secret,
                            }
                            : {}),
                    "content-type": "application/json",
                },
            });
            if (response.status === 200) {
                const body = await response.json();
                res(body);
            }
            else {
                try {
                    const result = await response.json();
                    rej({ status: response.status, result });
                }
                catch (error) {
                    rej({
                        status: response.status,
                        result: {
                            error: "Failed to get response body",
                            trace: [JSON.stringify(error)],
                        },
                        error,
                    });
                }
            }
        }
        catch (error) {
            rej({
                status: 1,
                result: {
                    error: "Request failed with error",
                    trace: [JSON.stringify(error)],
                },
                error,
            });
        }
    });
    const auth = async (type, params) => await request("/auth", {
        type,
        params,
    });
    const user = async (type, params) => await request("/user", { type, params });
    const read = async (type, params) => await request("/read", { type, params });
    const write = async (type, params) => await request("/write", { type, params });
    const execute = async (type, params) => await request("/execute", { type, params });
    const core_version = () => read("GetVersion", {}).then((res) => res.version);
    return {
        /**
         * Call the `/auth` api.
         *
         * ```
         * const login_options = await komodo.auth("GetLoginOptions", {});
         * ```
         *
         * https://docs.rs/komodo_client/latest/komodo_client/api/auth/index.html
         */
        auth,
        /**
         * Call the `/user` api.
         *
         * ```
         * const { key, secret } = await komodo.user("CreateApiKey", {
         *   name: "my-api-key"
         * });
         * ```
         *
         * https://docs.rs/komodo_client/latest/komodo_client/api/user/index.html
         */
        user,
        /**
         * Call the `/read` api.
         *
         * ```
         * const stack = await komodo.read("GetStack", {
         *   stack: "my-stack"
         * });
         * ```
         *
         * https://docs.rs/komodo_client/latest/komodo_client/api/read/index.html
         */
        read,
        /**
         * Call the `/write` api.
         *
         * ```
         * const build = await komodo.write("UpdateBuild", {
         *   id: "my-build",
         *   config: {
         *     version: "1.0.4"
         *   }
         * });
         * ```
         *
         * https://docs.rs/komodo_client/latest/komodo_client/api/write/index.html
         */
        write,
        /**
         * Call the `/execute` api.
         *
         * ```
         * const update = await komodo.execute("DeployStack", {
         *   stack: "my-stack"
         * });
         * ```
         *
         * https://docs.rs/komodo_client/latest/komodo_client/api/execute/index.html
         */
        execute,
        /** Returns the version of Komodo Core the client is calling to. */
        core_version,
    };
}
