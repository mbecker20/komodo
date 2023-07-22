import { ReadRequest } from "@monitor/client/dist/types";
import { client } from "./main";
import { useQuery, useMutation } from "@tanstack/react-query";

export const useRead = <T extends ReadRequest>(req: T) =>
  useQuery([req], () => client.read(req));

export const useUser = () => useRead({ type: "GetUser", params: {} });

export const useLogin = () =>
  useMutation(client.login, {
    onSuccess: (jwt) => localStorage.setItem("monitor-auth-token", jwt ?? ""),
  });
