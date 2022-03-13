import { client, redirectTo, URL } from "..";
import { User } from "@oauth2/types";
import axios from "axios";

export function combineClasses(...classes: (string | undefined)[]) {
  return classes.filter((c) => (c ? true : false)).join(" ");
}

export function getAuthProvider(user: User) {
  if (user.githubID) return "Github";
  else if (user.googleID) return "Google";
  else return "Local";
}

export function getRedirectTo() {
  const params = new URLSearchParams(location.search);
  const redirect = params.get("redirect");
  if (redirect) {
    if (redirect === "consumer") {
      return {
        service: "consumer",
        url: "http://localhost:3000"
      }
    }
  }
}

export function loginGithub() {
  window.location.replace(
    `${URL}/login/github${redirectTo ? "/" + redirectTo.service : ""}`
  );
}

export function loginGoogle() {
  window.location.replace(
    `${URL}/login/google${
      redirectTo ? "/" + redirectTo.service : ""
    }`
  );
}

export function inPx(num: number) {
  return `${num}px`;
}