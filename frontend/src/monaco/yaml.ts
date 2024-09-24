import * as monaco from "monaco-editor";

const yaml_conf: monaco.languages.LanguageConfiguration = {
  comments: {
    lineComment: "#",
  },
  brackets: [
    ["{", "}"],
    ["[", "]"],
  ],
  autoClosingPairs: [
    { open: "{", close: "}" },
    { open: "[", close: "]" },
    { open: '"', close: '"' },
    { open: "'", close: "'" },
  ],
  surroundingPairs: [
    { open: "{", close: "}" },
    { open: "[", close: "]" },
    { open: '"', close: '"' },
    { open: "'", close: "'" },
  ],
  indentationRules: {
    increaseIndentPattern: /^.*(\{[^}]*|\[[^\]]*)$/,
    decreaseIndentPattern: /^\s*[}\]],?\s*$/,
  },
};

const yaml_language = <monaco.languages.IMonarchLanguage>{
  defaultToken: "",
  tokenPostfix: ".yaml",

  // Common regular expressions
  escapes:
    /\\(?:[abfnrtv\\"']|x[0-9A-Fa-f]{1,4}|u[0-9A-Fa-f]{4}|U[0-9A-Fa-f]{8})/,

  // The main tokenizer for YAML
  tokenizer: {
    root: [
      { include: "@whitespace" },
      { include: "@comments" },
      { include: "@keys" },
      { include: "@numbers" },
      { include: "@booleans" },
      { include: "@strings" },
      { include: "@constants" },
    ],

    whitespace: [[/[ \t\r\n]+/, ""]],

    comments: [[/#.*$/, "comment"]],

    keys: [[/([^\s\[\]{},"']+)(\s*)(:)/, ["key", "", "delimiter"]]],

    numbers: [
      [/\b\d+\.\d*\b/, "number.float"],
      [/\b0x[0-9a-fA-F]+\b/, "number.hex"],
      [/\b\d+\b/, "number"],
    ],

    booleans: [[/\b(true|false|yes|no|on|off)\b/, "constant.language.boolean"]],

    strings: [
      [/"([^"\\]|\\.)*$/, "string.invalid"], // non-terminated string
      [/'([^'\\]|\\.)*$/, "string.invalid"], // non-terminated string
      [/"/, "string", "@string_double"],
      [/'/, "string", "@string_single"],
    ],

    string_double: [
      [/[^\\"]+/, "string"],
      [/@escapes/, "string.escape"],
      [/\\./, "string.escape.invalid"],
      [/"/, "string", "@pop"],
    ],

    string_single: [
      [/[^\\']+/, "string"],
      [/@escapes/, "string.escape"],
      [/\\./, "string.escape.invalid"],
      [/'/, "string", "@pop"],
    ],

    constants: [[/\b(null|~)\b/, "constant.language.null"]],
  },
};

monaco.languages.register({ id: "yaml" });
monaco.languages.setMonarchTokensProvider("yaml", yaml_language);
monaco.languages.setLanguageConfiguration("yaml", yaml_conf);
