import { Collection, User } from "@monitor/types";

export function combineClasses(...classes: (string | false | undefined)[]) {
  return classes.filter((c) => (c ? true : false)).join(" ");
}

export function inPx(num: number) {
  return `${num}px`;
}

export function getAuthProvider(user: User) {
  if (user.githubID) return "Github";
  else return "Local";
}

export function generateQuery(query?: Collection<string | number | undefined>) {
  if (query) {
    const q = Object.keys(query)
      .filter((key) => query[key] !== undefined)
      .map((key) => key + "=" + query[key])
      .join("&");
    return q && `?${q}`;
  } else return "";
}

export function readablePermissions(permissions: number) {
  switch (permissions) {
    case 0:
      return "view only";

    case 1:
      return "user";

    case 2:
      return "admin";

    default:
      return "view only";
  }
}

export function readableTimestamp(unixTimeInSecs: number) {
  const date = new Date(unixTimeInSecs * 1000);
  const hours24 = date.getHours();
  let hours = hours24 % 12;
  if (hours === 0) hours = 12;
  const pm = hours24 > 11;
  const minutes = date.getMinutes();
  return `${date.getMonth() + 1}/${date.getDate()} ${hours}:${
    minutes > 9 ? minutes : "0" + minutes
  } ${pm ? "PM" : "AM"}`;
}

export function readableOperation(operation: string) {
  return operation.toLowerCase().replaceAll("_", " ");
}

export function deploymentStatusClass(
  status: "not deployed" | "created" | "running" | "exited" | "unknown"
) {
  switch (status) {
    case "running":
      return "green";
    case "exited":
      return "red";
    default:
      return "blue";
  }
}

export function deploymentHeaderStatusClass(
  status: "not deployed" | "created" | "running" | "exited" | "unknown",
  themeClass: () => string | undefined
) {
  switch (status) {
    case "running":
      return combineClasses("running", themeClass());
    case "exited":
      return combineClasses("exited", themeClass());
  }
}

export function serverStatusClass(
  status: "OK" | "NOT OK" | "DISABLED",
  themeClass: () => string | undefined
) {
  switch (status) {
    case "OK":
      return combineClasses("running", themeClass());
    case "NOT OK":
      return combineClasses("exited", themeClass());
  }
}

export function copyToClipboard(text: string) {
  navigator.clipboard.writeText(text);
}

export function applyDarkTheme(element: Element) {
  element.classList.add("dark");
  for (let i = 0; i < element.children.length; i++) {
    applyDarkTheme(element.children[i]);
  }
}

export function removeDarkTheme(element: Element) {
  element.classList.remove("dark");
  for (let i = 0; i < element.children.length; i++) {
    removeDarkTheme(element.children[i]);
  }
}

export function validatePercentage(perc: string) {
  // validates that a string represents a percentage
  const percNum = Number(perc);
  return !isNaN(percNum) && percNum > 0 && percNum < 100;
}
