import { createContext, createMemo, createResource, ParentComponent, Setter, useContext } from "solid-js";
import { client } from "..";
import { User } from "../types";

export const LOGGED_IN_ENABLED = "LOGGED_IN_ENABLED";
export const LOGGED_IN_DISABLED = "LOGGED_IN_DISABLED";
export const SIGNED_OUT = "SIGNED_OUT";
export const UNKNOWN = "UNKNOWN";

export type UserState = {
  user: () => User;
  setUser: Setter<false | User | undefined>;
  logout: () => void;
  username: () => string;
  loginStatus: () =>
    | "LOGGED_IN_ENABLED"
    | "LOGGED_IN_DISABLED"
    | "SIGNED_OUT"
    | "UNKNOWN";
  reloadUser: () => void;
};

const UserContext = createContext<UserState>();

export const UserProvider: ParentComponent = (p) => {
  const [user, { mutate, refetch }] = createResource(() => client.get_user());
  const logout = async () => {
    client.logout();
    mutate(false);
  };
  const username = () => {
    return (user() as User)?.username!;
  };
  const loginStatus = createMemo(() => {
    const _user = user();
    if (_user) {
      if (_user.enabled) {
        return LOGGED_IN_ENABLED;
      } else {
        return LOGGED_IN_DISABLED;
      }
    } else if (_user === false) return SIGNED_OUT;
    else return UNKNOWN;
  });
  const context: UserState = {
    user: () => user() as User,
    setUser: mutate,
    logout,
    username,
    loginStatus,
    reloadUser: refetch,
  };
  return (
    <UserContext.Provider value={context}>{p.children}</UserContext.Provider>
  );
};

export function useUser(): UserState {
  return useContext(UserContext) as UserState;
}