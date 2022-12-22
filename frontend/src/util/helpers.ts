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

export function intoCollection<T>(arr: T[]): Record<string, T> {
  return Object.fromEntries(arr.map((item) => [(item as any)._id.$oid, item]));
}