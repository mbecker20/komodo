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

  escapes: /\\(?:[btnfr\"\'\\\/]|u[0-9A-Fa-f]{4}|U[0-9A-Fa-f]{8})/,

  tokenizer: {
    root: [
      // Comments
      [/\s*((#).*)$/, "comment"],

      // Table Definitions
      [
        /^\s*(\[\[)([^[\]]+)(\]\])/,
        [
          "punctuation.definition.array.table.toml",
          // "support.type.property-name.array.toml",
          "entity.other.attribute-name.table.array.toml",
          "punctuation.definition.array.table.toml",
        ],
      ],
      [
        /^\s*(\[)([^[\]]+)(\])/,
        [
          "punctuation.definition.table.toml",
          // "support.type.property-name.table.toml",
          "entity.other.attribute-name.table.toml",
          "punctuation.definition.table.toml",
        ],
      ],

      // Inline tables
      [
        /\{/,
        {
          token: "punctuation.definition.table.inline.toml",
          next: "@inlineTable",
        },
      ],

      // Entry (Key = Value)
      [
        /\s*((?:(?:(?:[A-Za-z0-9_+-]+)|(?:\"[^\"]+\")|(?:'[^']+'))\s*\.?\s*)+)\s*(=)/,
        ["support.type.property-name.toml", "delimiter"],
      ],

      // Values (booleans, numbers, dates, strings, arrays)
      { include: "@values" },
    ],

    // Inline Table
    inlineTable: [
      [
        /\}/,
        { token: "punctuation.definition.table.inline.toml", next: "@pop" },
      ],
      { include: "@comments" },
      [/,/, "punctuation.separator.table.inline.toml"],
      { include: "@values" },
    ],

    // Values (Strings, Numbers, Booleans, Dates, Arrays)
    values: [
      // Triple quoted string (basic)
      [/"""/, { token: "string", next: "@tripleStringWithYamlEnv" }],

      // Single quoted string
      [
        /"/,
        { token: "string.quoted.single.basic.line.toml", next: "@basicString" },
      ],

      // Triple quoted literal string
      [
        /'''/,
        {
          token: "string.quoted.triple.literal.block.toml",
          next: "@literalStringTriple",
        },
      ],

      // Single quoted literal string
      [
        /'/,
        {
          token: "string.quoted.single.literal.line.toml",
          next: "@literalStringSingle",
        },
      ],

      // Dates and Times
      [
        /\d{4}-\d{2}-\d{2}[Tt ]\d{2}:\d{2}:\d{2}(?:\.\d+)?(?:Z|[+-]\d{2}:\d{2})/,
        "constant.other.time.datetime.offset.toml",
      ],
      [
        /\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}(?:\.\d+)?/,
        "constant.other.time.datetime.local.toml",
      ],
      [/\d{4}-\d{2}-\d{2}/, "constant.other.time.date.toml"],
      [/\d{2}:\d{2}:\d{2}(?:\.\d+)?/, "constant.other.time.time.toml"],

      // Booleans
      [/\b(true|false)\b/, "constant.language.boolean.toml"],

      // Numbers
      [
        /[+-]?(0|[1-9][0-9_]*)(\.[0-9_]+)?([eE][+-]?[0-9_]+)?/,
        "constant.numeric.float.toml",
      ],
      [
        /[+-]?(0x[0-9A-Fa-f_]+|0o[0-7_]+|0b[01_]+)/,
        "constant.numeric.hex.toml",
      ],

      // Arrays
      [/\[/, { token: "punctuation.definition.array.toml", next: "@array" }],
    ],

    // Basic quoted string
    basicString: [
      [/[^\\"]+/, "string"],
      [/@escapes/, "constant.character.escape.toml"],
      [/\\./, "invalid"],
      [/"/, { token: "string.quoted.single.basic.line.toml", next: "@pop" }],
    ],

    // Literal triple quoted string
    literalStringTriple: [
      [/[^']+/, "string"],
      [/'/, { token: "string.quoted.triple.literal.block.toml", next: "@pop" }],
    ],

    // Literal single quoted string
    literalStringSingle: [
      [/[^']+/, "string"],
      [/'/, { token: "string.quoted.single.literal.line.toml", next: "@pop" }],
    ],

    // Arrays
    array: [
      [/\]/, { token: "punctuation.definition.array.toml", next: "@pop" }],
      [/,/, "punctuation.separator.array.toml"],
      { include: "@values" },
    ],

    // Handle whitespace and comments
    whitespace: [[/\s+/, ""]],
    comments: [[/\s*((#).*)$/, "comment.line.number-sign.toml"]],

    // CUSTOM STUFF FOR YAML / ENV IN TRIPLE STRING

    tripleStringWithYamlEnv: [
      [/"""/, { token: "string", next: "@pop" }],
      { include: "@yamlTokenizer" }, // YAML inside triple quotes
      { include: "@envVariableTokenizer" }, // Environment Variable parsing inside triple quotes
    ],

    // YAML Tokenizer for inside triple quotes
    yamlTokenizer: [
      { include: "@yaml_whitespace" },
      { include: "@yaml_comments" },
      { include: "@yaml_keys" },
      { include: "@yaml_numbers" },
      { include: "@yaml_booleans" },
      { include: "@yaml_strings" },
      { include: "@yaml_constants" },
    ],

    // Environment Variable Tokenizer
    envVariableTokenizer: [
      [
        /(\s*-*\s*)([A-Za-z0-9_]+)(\s*)(=|:)(\s*)/,
        ["", "key", "", "operator.assignment", ""],
      ],
      { include: "@yamlTokenizer" }, // Use YAML tokenizer for EnvVar values
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
      [/"([^"\\]|\\.)*$/, "string.invalid"], // Non-terminated string
      [/'([^'\\]|\\.)*$/, "string.invalid"], // Non-terminated string
      [/"/, "string", "@yaml_string_double"],
      [/'/, "string", "@yaml_string_single"],
    ],
    yaml_string_double: [
      [/[^\\"]+/, "string"],
      [/@escapes/, "string.escape"],
      [/\\./, "string.escape.invalid"],
      [/"/, { token: "string", next: "@pop" }],
    ],
    yaml_string_single: [
      [/[^\\']+/, "string"],
      [/@escapes/, "string.escape"],
      [/\\./, "string.escape.invalid"],
      [/'/, { token: "string", next: "@pop" }],
    ],
    yaml_constants: [[/\b(null|~)\b/, "constant.language.null"]],
  },
};

monaco.languages.register({ id: "toml" });
monaco.languages.setLanguageConfiguration("toml", toml_conf);
monaco.languages.setMonarchTokensProvider("toml", toml_language);

// /// V2: Toml + Yaml + Env Vars
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
//       // triple-quoted string support with YAML and Environment Variable syntax inside
//       [/"""/, { token: "string", next: "@tripleStringWithYamlEnv" }],

//       // single-quoted strings and recover non-terminated strings
//       [/"([^"\\]|\\.)*$/, "string.invalid"], // non-terminated string
//       [/"/, "string", "@string_double"],

//       // escape sequences
//       [/@escapes/, "constant.character.escape.toml"],
//       [/\\./, "invalid.illegal.escape.toml"],
//     ],

//     tripleStringWithYamlEnv: [
//       // End of triple-quoted string
//       [/"""/, { token: "string", next: "@pop" }],
//       // Match content within triple-quoted string as YAML or Environment Variables
//       { include: "@yamlTokenizer" },
//       { include: "@envVariableTokenizer" },
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
//       { include: "@yaml_whitespace" },
//       { include: "@yaml_comment" },
//       { include: "@yaml_keys" },
//       { include: "@yaml_numbers" },
//       { include: "@yaml_booleans" },
//       { include: "@yaml_strings" },
//       { include: "@yaml_constants" },
//     ],

//     envVariableTokenizer: [
//       // Environment Variable specific content (KEY = VALUE)
//       [
//         /(\s*-*\s*)([A-Za-z0-9_]+)(\s*)(=|:)(\s*)/,
//         [
//           "", // Optional leading dash or space
//           "key", // Key (Environment Variable)
//           "", // Whitespace
//           "operator.assignment", // Equals sign (=)
//           "", // Whitespace
//         ],
//       ],
//       { include: "@yamlTokenizer" }, // Use YAML parsing for values
//     ],

//     yaml_whitespace: [[/[ \t\r\n]+/, ""]],

//     yaml_comment: [[/#.*$/, "comment"]],

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
//       [/"([^"\\]|\\.)*$/, "string.invalid"], // Non-terminated string
//       [/'([^'\\]|\\.)*$/, "string.invalid"], // Non-terminated string
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
