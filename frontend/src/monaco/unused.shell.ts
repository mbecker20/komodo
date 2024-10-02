import * as monaco from "monaco-editor";

const shell_conf: monaco.languages.LanguageConfiguration = {
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
  ],
  surroundingPairs: [
    { open: "{", close: "}" },
    { open: "[", close: "]" },
    { open: "(", close: ")" },
    { open: '"', close: '"' },
    { open: "'", close: "'" },
  ],
};

const shell_language = <monaco.languages.IMonarchLanguage>{
  defaultToken: "",
  ignoreCase: true,

  keywords: [
    "if",
    "then",
    "else",
    "fi",
    "for",
    "while",
    "do",
    "done",
    "echo",
    "return",
    "exit",
    "in",
    "function",
  ],

  operators: [
    "&&",
    "||",
    "==",
    "!=",
    "<",
    ">",
    ">=",
    "<=",
    "=",
    "+=",
    "-=",
    "*=",
    "/=",
    "%=",
  ],

  brackets: [
    { open: "{", close: "}", token: "delimiter.curly" },
    { open: "[", close: "]", token: "delimiter.square" },
    { open: "(", close: ")", token: "delimiter.parenthesis" },
  ],

  tokenizer: {
    root: [
      // Comments
      [/#.*$/, "comment"],

      // Keywords and control flow, use \b for word boundaries
      [
        /\b(if|then|else|fi|for|while|do|done|echo|return|exit|in|function)\b/,
        "keyword",
      ],

      // Operators
      [/\b(and|or|not|eq|ne|lt|gt|le|ge)\b/, "operator"],

      // Strings
      [/"/, "string", "@string_double"],
      [/'/, "string", "@string_single"],

      // Variables and parameter expansions
      [/\$[a-zA-Z_][a-zA-Z0-9_]*/, "variable"], // Variables like $VAR
      [/\$\{[^}]+\}/, "variable"], // Variables like ${VAR}

      // Numbers
      [/\b\d+(\.\d+)?\b/, "number"],

      // Brackets
      [/[{}[\]()]/, "@brackets"],

      // Operators
      [/[<>=!%&+\-*/|^~]+/, "operator"],

      // Whitespace
      { include: "@whitespace" },
    ],

    string_double: [
      [/[^\\"]+/, "string"],
      [/\\./, "string.escape"],
      [/"/, { token: "string", next: "@pop" }],
    ],

    string_single: [
      [/[^\\']+/, "string"],
      [/\\./, "string.escape"],
      [/'/, { token: "string", next: "@pop" }],
    ],

    whitespace: [[/\s+/, "white"]],
  },
};

// Register the shell language
monaco.languages.register({ id: "shell" });
monaco.languages.setLanguageConfiguration("shell", shell_conf);
monaco.languages.setMonarchTokensProvider("shell", shell_language);
