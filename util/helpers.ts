import {
  Collection,
  CommandLogError,
  EnvironmentVar,
  Log,
} from "@monitor/types";

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

export function parseEnvVarseToDotEnv(envVars: EnvironmentVar[]) {
  return envVars.reduce(
    (prev, { variable, value }) =>
      prev + (prev ? "\n" : "") + `${variable}=${value}`,
    ""
  );
}

function shouldKeepLine(line: string) {
  if (line.length === 0) {
    return false;
  }
  let firstIndex = -1;
  for (let i = 0; i < line.length; i++) {
    if (line[i] !== " ") {
      firstIndex = i;
      break;
    }
  }
  if (firstIndex === -1) {
    return false;
  }
  if (line[firstIndex] === "#") {
    return false;
  }
  return true;
}

export function parseDotEnvToEnvVars(env: string) {
  return env
    .split("\n")
    .filter((line) => shouldKeepLine(line))
    .map((entry) => {
      const [first, ...rest] = entry.replaceAll('"', "").split("=");
      return [first, rest.join("=")];
    })
    .map(([variable, value]) => ({ variable, value }));
}

const FIVE_MIN_MS = 1000 * 60 * 5;
const ONE_HOUR_MS = FIVE_MIN_MS * 12;
const ONE_DAY_MS = ONE_HOUR_MS * 24;

export function waitUntilNextFiveMinute() {
  const ts = Date.now();
  const timeToWait = FIVE_MIN_MS - ts % FIVE_MIN_MS;
  return new Promise<void>((res) => {
    setInterval(() => res(), timeToWait);
  });
}

export function waitUntilUTCHour(hour: number) {
  const ts = Date.now();
  const timeAfterMidnight = ts % ONE_DAY_MS;
  const targetTime = hour * ONE_HOUR_MS;
  const timeToWait =
    timeAfterMidnight < targetTime
      ? targetTime - timeAfterMidnight
      : ONE_DAY_MS - timeAfterMidnight + targetTime;
  return new Promise<void>((res) => {
    setInterval(() => res(), timeToWait);
  });
}
