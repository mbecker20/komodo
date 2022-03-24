export function toDashedName(name: string) {
  return name.toLowerCase().replaceAll(" ", "-");
}

export function bound(num: number, min: number, max: number) {
  return Math.min(max, Math.max(min, num));
}

export function prettyStringify(json: any): string {
  return JSON.stringify(json, undefined, 2);
}

export function timestamp() {
  return Math.floor(Date.now() / 1000);
}