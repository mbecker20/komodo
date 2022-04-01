import { User } from "@monitor/types";
import { Component, createContext, createResource, Resource, Setter, useContext } from "solid-js";
import { client } from "..";

export const LOGGED_IN = "LOGGED_IN";
export const SIGNED_OUT = "SIGNED_OUT";
export const UNKNOWN = "UNKNOWN";

export type UserState = {
  user: () => User;
  setUser: Setter<false | User | undefined>;
  logout: () => void;
  username: () => string | undefined;
  permissions: () => number;
  loginStatus: () => "LOGGED_IN" | "SIGNED_OUT" | "UNKNOWN"
};

const UserContext = createContext<UserState>();

export const UserProvider: Component = (p) => {
  const [user, { mutate }] = createResource(() => client.getUser());
  const logout = async () => {
    client.logout();
    mutate(false);
  };
  const username = () => {
    if (user()) {
      return (user() as User).username;
    } else {
      return undefined;
    }
  };
  const loginStatus = () => {
    if (user()) return LOGGED_IN
    else if (user() === false) return SIGNED_OUT;
    else return UNKNOWN
  }
  const permissions = () => {
    if (user()) {
      return (user() as User).permissions!
    } else {
      return 0;
    }
  }
  const context: UserState = {
    user: () => user() as User,
    setUser: mutate,
    logout,
    username,
    permissions,
    loginStatus
  };
  return (
    <UserContext.Provider value={context}>{p.children}</UserContext.Provider>
  );
};

export function useUser(): UserState {
  return useContext(UserContext) as UserState;
}