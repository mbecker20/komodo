import * as monaco from "monaco-editor";

/// V2: Toml + Yaml + Env Vars
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
    { open: '"""', close: '"""' },
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
      // triple-quoted string support with YAML and Environment Variable syntax inside
      [/"""/, { token: "string", next: "@tripleStringWithYamlEnv" }],

      // single-quoted strings and recover non-terminated strings
      [/"([^"\\]|\\.)*$/, "string.invalid"], // non-terminated string
      [/"/, "string", "@string_double"],

      // escape sequences
      [/@escapes/, "constant.character.escape.toml"],
      [/\\./, "invalid.illegal.escape.toml"],
    ],

    tripleStringWithYamlEnv: [
      // End of triple-quoted string
      [/"""/, { token: "string", next: "@pop" }],
      // Match content within triple-quoted string as YAML or Environment Variables
      { include: "@yamlTokenizer" },
      { include: "@envVariableTokenizer" },
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

    yamlTokenizer: [
      // YAML specific content
      { include: "@yaml_whitespace" },
      { include: "@yaml_comments" },
      { include: "@yaml_keys" },
      { include: "@yaml_numbers" },
      { include: "@yaml_booleans" },
      { include: "@yaml_strings" },
      { include: "@yaml_constants" },
    ],

    envVariableTokenizer: [
      // Environment Variable specific content (KEY = VALUE)
      [
        /(\s*-*\s*)([A-Za-z0-9_]+)(\s*)(=|:)(\s*)/,
        [
          "", // Maybe starting -
          "key", // Use the same token as YAML keys for the environment variable key
          "", // Whitespace
          "operator.assignment", // Equals sign (=)
          "", // Whitespace
        ],
      ],

      // Parse value as yaml
      { include: "@yaml_whitespace" },
      { include: "@yaml_comments" },
      { include: "@yaml_keys" },
      { include: "@yaml_numbers" },
      { include: "@yaml_booleans" },
      { include: "@yaml_strings" },
      { include: "@yaml_constants" },
    ],

    yaml_whitespace: [[/[ \t\r\n]+/, ""]],

    yaml_comments: [[/#.*$/, "comment"]],

    yaml_keys: [[/([^\s\[\]{},"']+)(\s*)(:)/, ["key", "", "delimiter"]]],

    yaml_numbers: [
      [/\b\d+\.\d*\b/, "number.float"],
      [/\b0x[0-9a-fA-F]+\b/, "number.hex"],
      [/\b\d+\b/, "number"],
    ],

    yaml_booleans: [
      [/\b(true|false|yes|no|on|off)\b/, "constant.language.boolean"],
    ],

    yaml_strings: [
      [/"([^"\\]|\\.)*$/, "string.invalid"], // non-terminated string
      [/'([^'\\]|\\.)*$/, "string.invalid"], // non-terminated string
      [/"/, "string", "@yaml_string_double"],
      [/'/, "string", "@yaml_string_single"],
    ],

    yaml_string_double: [
      [/[^\\"]+/, "string"],
      [/@escapes/, "string.escape"],
      [/\\./, "string.escape.invalid"],
      [/"/, "string", "@pop"],
    ],

    yaml_string_single: [
      [/[^\\']+/, "string"],
      [/@escapes/, "string.escape"],
      [/\\./, "string.escape.invalid"],
      [/'/, "string", "@pop"],
    ],

    yaml_constants: [[/\b(null|~)\b/, "constant.language.null"]],
  },
};

monaco.languages.register({ id: "toml" });
monaco.languages.setLanguageConfiguration("toml", toml_conf);
monaco.languages.setMonarchTokensProvider("toml", toml_language);

// Version 1
// Just Toml + Yaml in """ strings
// const toml_conf: monaco.languages.LanguageConfiguration = {
//   comments: {
//     lineComment: "#",
//   },
//   brackets: [
//     ["{", "}"],
//     ["[", "]"],
//     ["(", ")"],
//   ],
//   autoClosingPairs: [
//     { open: "{", close: "}" },
//     { open: "[", close: "]" },
//     { open: "(", close: ")" },
//     { open: '"', close: '"' },
//     { open: "'", close: "'" },
//     { open: '"""', close: '"""' },
//   ],
//   surroundingPairs: [
//     { open: "{", close: "}" },
//     { open: "[", close: "]" },
//     { open: "(", close: ")" },
//     { open: '"', close: '"' },
//     { open: "'", close: "'" },
//     { open: '"""', close: '"""' },
//   ],
// };

// // Includes support for des
// const toml_language = <monaco.languages.IMonarchLanguage>{
//   defaultToken: "",
//   tokenPostfix: ".toml",

//   escapes:
//     /\\(?:[abfnrtv\\"']|x[0-9A-Fa-f]{1,4}|u[0-9A-Fa-f]{4}|U[0-9A-Fa-f]{8})/,

//   tokenizer: {
//     root: [
//       { include: "@comments" },
//       { include: "@tables" },
//       { include: "@keys" },
//       { include: "@whitespace" },
//       { include: "@dateTimeWithTz" },
//       { include: "@dateTime" },
//       { include: "@date" },
//       { include: "@float" },
//       { include: "@integer" },
//       { include: "@boolean" },
//       { include: "@string" },
//     ],

//     comments: [
//       [
//         /\s*((#).*)$/,
//         {
//           cases: {
//             $1: "comment.line.number-sign.toml",
//             $2: "punctuation.definition.comment.toml",
//           },
//         },
//       ],
//     ],

//     dateTimeWithTz: [
//       [
//         /(?<!\w)(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:\d{2}))(?!\w)/,
//         "constant.other.datetime-with-timezone.toml",
//       ],
//     ],

//     dateTime: [
//       [
//         /(\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?)/,
//         "constant.other.datetime.toml",
//       ],
//     ],

//     date: [[/(\d{4}-\d{2}-\d{2})/, "constant.other.date.toml"]],

//     float: [
//       [
//         /(?<!\w)([+-]?(0|([1-9](([0-9]|_[0-9])+)?))(?:(?:\.(0|([1-9](([0-9]|_[0-9])+)?)))?[eE][+-]?[1-9]_?[0-9]*|(?:\.[0-9_]*)))(?!\w)/,
//         "constant.numeric.float.toml",
//       ],
//     ],

//     integer: [
//       [
//         /(?<!\w)((?:[+-]?(0|([1-9](([0-9]|_[0-9])+)?))))(?!\w)/,
//         "constant.numeric.integer.toml",
//       ],
//     ],

//     boolean: [[/(?<!\w)(true|false)(?!\w)/, "constant.other.boolean.toml"]],

//     keys: [[/(^\w+)(\s*)(=)/, ["key", "", "delimiter"]]],

//     whitespace: [[/[ \t\r\n]+/, ""]],

//     string: [
//       // triple-quoted string support with YAML inside
//       [/"""/, { token: "string", next: "@tripleStringWithYaml" }],

//       // single-quoted strings and recover non-terminated strings
//       [/"([^"\\]|\\.)*$/, "string.invalid"], // non-terminated string
//       [/"/, "string", "@string_double"],

//       // escape sequences
//       [/@escapes/, "constant.character.escape.toml"],
//       [/\\./, "invalid.illegal.escape.toml"],
//     ],

//     tripleStringWithYaml: [
//       // End of triple-quoted string
//       [/"""/, { token: "string", next: "@pop" }],
//       // Match content within triple-quoted string as YAML
//       { include: "@yamlTokenizer" },
//     ],

//     string_double: [
//       [/[^\\"]+/, "string"],
//       [/\\./, "constant.character.escape.toml"],
//       [/"/, { token: "string", next: "@pop" }],
//     ],

//     tables: [
//       // standard table definitions
//       [
//         /^\s*(\[\[)([^[\]]+)(\]\])/,
//         [
//           "punctuation.definition.table.array.toml",
//           "entity.other.attribute-name.table.array.toml",
//           "punctuation.definition.table.array.toml",
//         ],
//       ],
//       [
//         /^\s*(\[)([^[\]]+)(\])/,
//         [
//           "punctuation.definition.table.toml",
//           "entity.other.attribute-name.table.toml",
//           "punctuation.definition.table.toml",
//         ],
//       ],
//     ],

//     yamlTokenizer: [
//       // YAML specific content (based on the YAML tokenizer)
//       { include: "@yaml_whitespace" },
//       { include: "@yaml_comments" },
//       { include: "@yaml_keys" },
//       { include: "@yaml_numbers" },
//       { include: "@yaml_booleans" },
//       { include: "@yaml_strings" },
//       { include: "@yaml_constants" },
//     ],

//     yaml_whitespace: [[/[ \t\r\n]+/, ""]],

//     yaml_comments: [[/#.*$/, "comment"]],

//     yaml_keys: [[/([^\s\[\]{},"']+)(\s*)(:)/, ["key", "", "delimiter"]]],

//     yaml_numbers: [
//       [/\b\d+\.\d*\b/, "number.float"],
//       [/\b0x[0-9a-fA-F]+\b/, "number.hex"],
//       [/\b\d+\b/, "number"],
//     ],

//     yaml_booleans: [
//       [/\b(true|false|yes|no|on|off)\b/, "constant.language.boolean"],
//     ],

//     yaml_strings: [
//       [/"([^"\\]|\\.)*$/, "string.invalid"], // non-terminated string
//       [/'([^'\\]|\\.)*$/, "string.invalid"], // non-terminated string
//       [/"/, "string", "@yaml_string_double"],
//       [/'/, "string", "@yaml_string_single"],
//     ],

//     yaml_string_double: [
//       [/[^\\"]+/, "string"],
//       [/@escapes/, "string.escape"],
//       [/\\./, "string.escape.invalid"],
//       [/"/, "string", "@pop"],
//     ],

//     yaml_string_single: [
//       [/[^\\']+/, "string"],
//       [/@escapes/, "string.escape"],
//       [/\\./, "string.escape.invalid"],
//       [/'/, "string", "@pop"],
//     ],

//     yaml_constants: [[/\b(null|~)\b/, "constant.language.null"]],
//   },
// };

// monaco.languages.register({ id: "toml" });
// monaco.languages.setLanguageConfiguration("toml", toml_conf);
// monaco.languages.setMonarchTokensProvider("toml", toml_language);
