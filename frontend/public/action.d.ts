import type { KomodoClient } from "./client/lib.d.ts";

declare global {
  var komodo: ReturnType<typeof KomodoClient>;
}

export {};
