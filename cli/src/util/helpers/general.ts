export function toDashedName(name: string) {
  return name.toLowerCase().replaceAll(" ", "-");
}

export function bound(num: number, min: number, max: number) {
  return Math.min(max, Math.max(min, num));
}