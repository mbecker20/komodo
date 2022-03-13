import { URL } from "..";
import { User } from "@monitor/types";

export function combineClasses(...classes: (string | undefined)[]) {
  return classes.filter((c) => (c ? true : false)).join(" ");
}

export function inPx(num: number) {
  return `${num}px`;
}

export function getAuthProvider(user: User) {
  if (user.githubID) return "Github";
  else return "Local";
}

export function loginGithub() {
  window.location.replace(
    `${URL}/login/github`
  );
}
