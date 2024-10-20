import { KomodoClient, Types as KomodoTypes } from "./client/lib.js";

declare global {
  var komodo: ReturnType<typeof KomodoClient>;
  export import Types = KomodoTypes;
}

export {}