import { loader } from "@monaco-editor/react";

import * as monaco from "monaco-editor";
import editorWorker from "monaco-editor/esm/vs/editor/editor.worker?worker";
import jsonWorker from "monaco-editor/esm/vs/language/json/json.worker?worker";
import cssWorker from "monaco-editor/esm/vs/language/css/css.worker?worker";
import htmlWorker from "monaco-editor/esm/vs/language/html/html.worker?worker";
import tsWorker from "monaco-editor/esm/vs/language/typescript/ts.worker?worker";

self.MonacoEnvironment = {
  getWorker(_, label) {
    if (label === "json") {
      return new jsonWorker();
    }
    if (label === "css" || label === "scss" || label === "less") {
      return new cssWorker();
    }
    if (label === "html" || label === "handlebars" || label === "razor") {
      return new htmlWorker();
    }
    if (label === "typescript" || label === "javascript") {
      return new tsWorker();
    }
    return new editorWorker();
  },
};

loader.config({ monaco });

monaco.editor.defineTheme("light", {
  base: "vs",
  inherit: true,
  rules: [],
  colors: {
    "editor.background": "#f7f8f9",
  },
});

monaco.editor.defineTheme("dark", {
  base: "vs-dark",
  inherit: true,
  rules: [],
  colors: {
    "editor.background": "#151b25",
  },
});

const toml_conf: monaco.languages.LanguageConfiguration = {
  comments: {
    lineComment: "#",
  },
  brackets: [
    ["{", "}"],
    ["[", "]"],
    ["(", ")"],
  ],
  autoClosingPairs: [
    { open: "{", close: "}" },
    { open: "[", close: "]" },
    { open: "(", close: ")" },
    { open: '"', close: '"' },
    { open: "'", close: "'" },
    { open: '"""', close: '"""' },
  ],
  surroundingPairs: [
    { open: "{", close: "}" },
    { open: "[", close: "]" },
    { open: "(", close: ")" },
    { open: '"', close: '"' },
    { open: "'", close: "'" },
  ],
};

const toml_language = <monaco.languages.IMonarchLanguage>{
  defaultToken: "",
  tokenPostfix: ".toml",

  escapes:
    /\\(?:[abfnrtv\\"']|x[0-9A-Fa-f]{1,4}|u[0-9A-Fa-f]{4}|U[0-9A-Fa-f]{8})/,

  tokenizer: {
    root: [
      { include: "@comments" },
      { include: "@tables" },
      { include: "@keys" },
      { include: "@whitespace" },
      { include: "@dateTimeWithTz" },
      { include: "@dateTime" },
      { include: "@date" },
      { include: "@float" },
      { include: "@integer" },
      { include: "@boolean" },
      { include: "@string" },
    ],

    comments: [
      [
        /\s*((#).*)$/,
        {
          cases: {
            $1: "comment.line.number-sign.toml",
            $2: "punctuation.definition.comment.toml",
          },
        },
      ],
    ],

    dateTimeWithTz: [
      [
        /(?<!\w)(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:\d{2}))(?!\w)/,
        "constant.other.datetime-with-timezone.toml",
      ],
    ],

    dateTime: [
      [
        /(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?)/,
        "constant.other.datetime.toml",
      ],
    ],

    date: [[/(\d{4}-\d{2}-\d{2})/, "constant.other.date.toml"]],

    float: [
      [
        /(?<!\w)([+-]?(0|([1-9](([0-9]|_[0-9])+)?))(?:(?:\.(0|([1-9](([0-9]|_[0-9])+)?)))?[eE][+-]?[1-9]_?[0-9]*|(?:\.[0-9_]*)))(?!\w)/,
        "constant.numeric.float.toml",
      ],
    ],

    integer: [
      [
        /(?<!\w)((?:[+-]?(0|([1-9](([0-9]|_[0-9])+)?))))(?!\w)/,
        "constant.numeric.integer.toml",
      ],
    ],

    boolean: [[/(?<!\w)(true|false)(?!\w)/, "constant.other.boolean.toml"]],

    keys: [[/(^\w+)(\s*)(=)/, ["key", "", "delimiter"]]],

    whitespace: [[/[ \t\r\n]+/, ""]],

    string: [
      // triple-quoted string support
      [/"""/, { token: "string", next: "@tripleString" }],

      // single-quoted strings and recover non-terminated strings
      [/"([^"\\]|\\.)*$/, "string.invalid"], // non-terminated string
      [/"/, "string", "@string_double"],

      // escape sequences
      [/@escapes/, "constant.character.escape.toml"],
      [/\\./, "invalid.illegal.escape.toml"],
    ],

    tripleString: [
      // End of triple-quoted string
      [/"""/, { token: "string", next: "@pop" }],
      // Match content within triple-quoted string
      [/[^"""]+/, "string"],
    ],

    string_double: [
      [/[^\\"]+/, "string"],
      [/\\./, "constant.character.escape.toml"],
      [/"/, { token: "string", next: "@pop" }],
    ],

    tables: [
      // standard table definitions
      [
        /^\s*(\[\[)([^[\]]+)(\]\])/,
        [
          "punctuation.definition.table.array.toml",
          "entity.other.attribute-name.table.array.toml",
          "punctuation.definition.table.array.toml",
        ],
      ],
      [
        /^\s*(\[)([^[\]]+)(\])/,
        [
          "punctuation.definition.table.toml",
          "entity.other.attribute-name.table.toml",
          "punctuation.definition.table.toml",
        ],
      ],
    ],
  },
};

monaco.languages.register({ id: "toml" });
monaco.languages.setLanguageConfiguration("toml", toml_conf);
monaco.languages.setMonarchTokensProvider("toml", toml_language);
