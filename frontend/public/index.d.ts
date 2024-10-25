import { KomodoClient, Types as KomodoTypes } from "./client/lib.js";
import "./deno.d.ts";

// =====================
// 🔴 YAML De/serializer
// =====================

// https://jsr.io/@std/yaml

export type YamlSchemaType =
  | "failsafe"
  | "json"
  | "core"
  | "default"
  | "extended";

export type YamlStyleVariant =
  | "lowercase"
  | "uppercase"
  | "camelcase"
  | "decimal"
  | "binary"
  | "octal"
  | "hexadecimal";

/** Options for `YAML.stringify` */
export type YamlStringifyOptions = {
  /**
   * Indentation width to use (in spaces).
   *
   * @default {2}
   */
  indent?: number;
  /**
   * When true, adds an indentation level to array elements.
   *
   * @default {true}
   */
  arrayIndent?: boolean;
  /**
   * Do not throw on invalid types (like function in the safe schema) and skip
   * pairs and single values with such types.
   *
   * @default {false}
   */
  skipInvalid?: boolean;
  /**
   * Specifies level of nesting, when to switch from block to flow style for
   * collections. `-1` means block style everywhere.
   *
   * @default {-1}
   */
  flowLevel?: number;
  /** Each tag may have own set of styles.	- "tag" => "style" map. */
  styles?: Record<string, YamlStyleVariant>;
  /**
   * Name of the schema to use.
   *
   * @default {"default"}
   */
  schema?: YamlSchemaType;
  /**
   * If true, sort keys when dumping YAML in ascending, ASCII character order.
   * If a function, use the function to sort the keys.
   * If a function is specified, the function must return a negative value
   * if first argument is less than second argument, zero if they're equal
   * and a positive value otherwise.
   *
   * @default {false}
   */
  sortKeys?: boolean | ((a: string, b: string) => number);
  /**
   * Set max line width.
   *
   * @default {80}
   */
  lineWidth?: number;
  /**
   * If false, don't convert duplicate objects into references.
   *
   * @default {true}
   */
  useAnchors?: boolean;
  /**
   * If false don't try to be compatible with older yaml versions.
   * Currently: don't quote "yes", "no" and so on,
   * as required for YAML 1.1.
   *
   * @default {true}
   */
  compatMode?: boolean;
  /**
   * If true flow sequences will be condensed, omitting the
   * space between `key: value` or `a, b`. Eg. `'[a,b]'` or `{a:{b:c}}`.
   * Can be useful when using yaml for pretty URL query params
   * as spaces are %-encoded.
   *
   * @default {false}
   */
  condenseFlow?: boolean;
};

/** Options for `YAML.parse` */
export interface YamlParseOptions {
  /**
   * Name of the schema to use.
   *
   * @default {"default"}
   */
  schema?: YamlSchemaType;
  /**
   * If `true`, duplicate keys will overwrite previous values. Otherwise,
   * duplicate keys will throw a {@linkcode SyntaxError}.
   *
   * @default {false}
   */
  allowDuplicateKeys?: boolean;
  /**
   * If defined, a function to call on warning messages taking an
   * {@linkcode Error} as its only argument.
   */
  onWarning?(error: Error): void;
}

export type YAML = {
  /**
   * Converts a JavaScript object or value to a YAML document string.
   *
   * @example Usage
   * ```ts
   * const data = { id: 1, name: "Alice" };
   *
   * const yaml = YAML.stringify(data);
   *
   * assertEquals(yaml, "id: 1\nname: Alice\n");
   * ```
   *
   * @throws {TypeError} If `data` contains invalid types.
   * @param data The data to serialize.
   * @param options The options for serialization.
   * @returns A YAML string.
   */
  stringify: (data: unknown, options?: YamlStringifyOptions) => string;
  /**
   * Parse and return a YAML string as a parsed YAML document object.
   *
   * Note: This does not support functions. Untrusted data is safe to parse.
   *
   * @example Usage
   * ```ts
   * const data = YAML.parse(`
   * id: 1
   * name: Alice
   * `);
   *
   * assertEquals(data, { id: 1, name: "Alice" });
   * ```
   *
   * @throws {SyntaxError} Throws error on invalid YAML.
   * @param content YAML string to parse.
   * @param options Parsing options.
   * @returns Parsed document.
   */
  parse: (content: string, options?: YamlParseOptions) => unknown;
  /**
   * Same as `YAML.parse`, but understands multi-document YAML sources, and
   * returns multiple parsed YAML document objects.
   *
   * @example Usage
   * ```ts
   * const data = YAML.parseAll(`
   * ---
   * id: 1
   * name: Alice
   * ---
   * id: 2
   * name: Bob
   * ---
   * id: 3
   * name: Eve
   * `);
   *
   * assertEquals(data, [ { id: 1, name: "Alice" }, { id: 2, name: "Bob" }, { id: 3, name: "Eve" }]);
   * ```
   *
   * @param content YAML string to parse.
   * @param options Parsing options.
   * @returns Array of parsed documents.
   */
  parseAll: (content: string, options?: YamlParseOptions) => unknown;
};

// =====================
// 🔴 TOML De/serializer
// =====================

// https://jsr.io/@std/toml

export interface TomlStringifyOptions {
  /**
   * Define if the keys should be aligned or not.
   *
   * @default {false}
   */
  keyAlignment?: boolean;
}

export type TOML = {
  /**
   * Converts an object to a [TOML string](https://toml.io).
   *
   * @example Usage
   * ```ts
   * const obj = {
   *   title: "TOML Example",
   *   owner: {
   *     name: "Bob",
   *     bio: "Bob is a cool guy",
   *  }
   * };
   *
   * const tomlString = TOML.stringify(obj);
   *
   * assertEquals(tomlString, `title = "TOML Example"\n\n[owner]\nname = "Bob"\nbio = "Bob is a cool guy"\n`);
   * ```
   * @param obj Source object
   * @param options Options for stringifying.
   * @returns TOML string
   */
  stringify: (
    obj: Record<string, unknown>,
    options?: TomlStringifyOptions
  ) => string;
  /**
   * Parses a [TOML string](https://toml.io) into an object.
   *
   * @example Usage
   * ```ts
   * const tomlString = `title = "TOML Example"
   * [owner]
   * name = "Alice"
   * bio = "Alice is a programmer."`;
   *
   * const obj = TOML.parse(tomlString);
   *
   * assertEquals(obj, { title: "TOML Example", owner: { name: "Alice", bio: "Alice is a programmer." } });
   * ```
   * @param tomlString TOML string to be parsed.
   * @returns The parsed JS object.
   */
  parse: (tomlString: string) => Record<string, unknown>;
};

declare global {
  /** Pre initialized Komodo client */
  var komodo: ReturnType<typeof KomodoClient>;
  /** YAML parsing utilities */
  var YAML: YAML;
  /** TOML parsing utilities */
  var TOML: TOML;
  /** All Komodo Types */
  export import Types = KomodoTypes;
}
