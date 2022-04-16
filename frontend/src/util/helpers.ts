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

export function objFrom2Arrays<T>(
  keys: string[],
  entries: T[]
): Collection<T | undefined> {
  return Object.fromEntries(
    keys.map((id, index) => {
      return [id, entries[index]];
    })
  );
}

export function filterOutFromObj<T>(obj: T, idsToFilterOut: string[]) {
  return Object.fromEntries(
    Object.entries(obj).filter((entry) => {
      return !idsToFilterOut.includes(entry[0]);
    })
  );
}

export function keepOnlyInObj<T>(obj: T, idsToKeep: string[]) {
  return Object.fromEntries(
    Object.entries(obj).filter((entry) => {
      return idsToKeep.includes(entry[0]);
    })
  ) as T;
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
  status: "not deployed" | "created" | "running" | "exited"
) {
  switch (status) {
    case "running":
      return "running";
    case "exited":
      return "exited";
  }
}

export function serverStatusClass(
  status: "OK" | "NOT OK" | "DISABLED"
) {
  switch (status) {
    case "OK":
      return "running";
    case "NOT OK":
      return "exited";
  }
}

export function copyToClipboard(text: string) {
  navigator.clipboard.writeText(text);
}
