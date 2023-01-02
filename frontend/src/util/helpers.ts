import { DockerContainerState, ServerStatus } from "../types";

export function combineClasses(...classes: (string | false | undefined)[]) {
  return classes.filter((c) => (c ? true : false)).join(" ");
}

export function inPx(num: number) {
  return `${num}px`;
}

export type QueryObject = Record<string, string | number | undefined>;

export function generateQuery(query?: QueryObject) {
  if (query) {
    const q = Object.keys(query)
      .filter((key) => query[key] !== undefined)
      .map((key) => key + "=" + query[key])
      .join("&");
    return q && `?${q}`;
  } else return "";
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

export function readableMonitorTimestamp(rfc3339_ts: string) {
  const date = new Date(rfc3339_ts);
  const hours24 = date.getHours();
  let hours = hours24 % 12;
  if (hours === 0) hours = 12;
  const pm = hours24 > 11;
  const minutes = date.getMinutes();
  return `${date.getMonth() + 1}/${date.getDate()} ${hours}:${
    minutes > 9 ? minutes : "0" + minutes
  } ${pm ? "PM" : "AM"}`;
}

export function readableDuration(start_ts: string, end_ts: string) {
  const start = new Date(start_ts);
  const end = new Date(end_ts);
  const durr = end.getTime() - start.getTime();
  const seconds = (durr / 1000).toFixed(1);
  return `${seconds} seconds`
}

export function validatePercentage(perc: string) {
  // validates that a string represents a percentage
  const percNum = Number(perc);
  return !isNaN(percNum) && percNum > 0 && percNum < 100;
}

export function filterOutFromObj<T>(obj: T, idsToFilterOut: string[]) {
  return Object.fromEntries(
    Object.entries(obj as any).filter((entry) => {
      return !idsToFilterOut.includes(entry[0]);
    })
  ) as T;
}

export function keepOnlyInObj<T>(obj: T, idsToKeep: string[]) {
  return Object.fromEntries(
    Object.entries(obj as any).filter((entry) => {
      return idsToKeep.includes(entry[0]);
    })
  ) as T;
}

export function getNestedEntry(obj: any, idPath: string[]): any {
  if (idPath.length === 0) {
    return obj;
  } else {
    return getNestedEntry(obj[idPath[0]], idPath.slice(1));
  }
}

export function intoCollection<T>(arr: T[], idPath: string[]): Record<string, T> {
  return Object.fromEntries(arr.map((item) => [getNestedEntry(item, idPath), item]));
}

export function getId(entity: any): string {
  return entity._id.$oid
}

export function deploymentStateClass(state: DockerContainerState) {
  switch (state) {
    case DockerContainerState.Running:
      return "green";
    case DockerContainerState.Exited:
      return "red";
    default:
      return "blue";
  }
}

export function serverStatusClass(
  status: ServerStatus
) {
  if (status === ServerStatus.Ok) {
    return "running"
  } else if (status === ServerStatus.NotOk) {
    return "exited"
  }
}