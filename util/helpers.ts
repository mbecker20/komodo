import { Collection, CommandLogError, Log } from "@monitor/types";

export function objFrom2Arrays<T>(keys: string[], entries: T[]): Collection<T> {
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
  ) as T;
}

export function keepOnlyInObj<T>(obj: T, idsToKeep: string[]) {
  return Object.fromEntries(
    Object.entries(obj).filter((entry) => {
      return idsToKeep.includes(entry[0]);
    })
  ) as T;
}

export function filterOutUndefined<T>(obj: T) {
  return Object.fromEntries(
    Object.entries(obj).filter(([_, entry]) => entry !== undefined)
  ) as T;
}

export function intoCollection<T>(arr: T[], field = "_id"): Collection<T> {
  return Object.fromEntries(arr.map((item) => [(item as any)[field], item]));
}

export function timestamp() {
  return Math.floor(Date.now() / 1000);
}

export function combineLogs(log0: Log, log1: Log): Log {
  return {
    stdout:
      (log0.stdout ? log0.stdout : "") +
      (log0.stdout && log1.stdout ? ", " : "") +
      (log1.stdout ? log1.stdout : ""),
    stderr:
      (log0.stderr ? log0.stderr : "") +
      (log0.stderr && log1.stderr ? ", " : "") +
      (log1.stderr ? log1.stderr : ""),
  };
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

export function mergeCommandLogError(
  ...cle: { name: string; cle: CommandLogError | undefined }[]
) {
  const _cle = cle.filter((cle) => cle.cle) as {
    name: string;
    cle: CommandLogError;
  }[];
  const moreThanOne = _cle.length > 1;
  const command = _cle.reduce((prev, curr) => {
    return (
      prev +
      (prev && "\n\n") +
      `${moreThanOne ? curr.name + ": " : ""}${curr.cle.command}`
    );
  }, "");
  const log = _cle.reduce(
    (log, curr) => {
      log.stdout =
        log.stdout +
        (log.stdout && (curr.cle.log.stdout ? "\n\n" : "")) +
        (curr.cle.log.stdout
          ? `${moreThanOne ? curr.name + ":\n" : ""}${curr.cle.log.stdout}`
          : "");
      log.stderr =
        log.stderr +
        (log.stderr && (curr.cle.log.stderr ? "\n\n" : "")) +
        (curr.cle.log.stderr
          ? `${moreThanOne ? curr.name + ":\n" : ""}${curr.cle.log.stderr}`
          : "");
      return log;
    },
    { stdout: "", stderr: "" }
  );
  const isError = _cle.filter((cle) => cle.cle.isError).length > 0;
  return {
    command,
    log,
    isError,
  };
}

export function trailingSlash(str: string) {
  return str[str.length - 1] === "/" ? str : str + "/";
}

export function prettyStringify(json: any) {
  return JSON.stringify(json, undefined, 2);
}

export function convertFieldsToNumbers(obj: any): {
  [key: string]: number;
} {
  return objMap(obj, (entry) => Number(entry));
}

export function objMap<T>(obj: { [key: string]: T }, map: (entry: T) => any) {
  return Object.fromEntries(
    Object.entries(obj).map((entry) => [entry[0], map(entry[1])])
  );
}