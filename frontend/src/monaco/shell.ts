import * as monaco from "monaco-editor";

const shell_conf: monaco.languages.LanguageConfiguration = {
  comments: {
    lineComment: "#",
  },
  brackets: [
    ["{", "}"],
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
  tokenPostfix: ".shell",

  keywords: [
    "if",
    "then",
    "else",
    "fi",
    "for",
    "while",
    "do",
    "done",
    "in",
    "case",
    "esac",
    "function",
  ],

  builtins: [
    "echo",
    "cd",
    "pwd",
    "ls",
    "rm",
    "cp",
    "mv",
    "cat",
    "grep",
    "find",
    "chmod",
    "chown",
    "mkdir",
    "rmdir",
    "touch",
    "exit",
    "source",
  ],

  // Common operators
  symbols: /[=><!~?:&|+\-*\/\^%]+/,

  tokenizer: {
    root: [
      // Comments
      [/#.*$/, "comment"],

      // Keywords
      [
        /\b(if|then|else|fi|for|while|do|done|in|case|esac|function)\b/,
        "keyword",
      ],

      // Built-in commands
      [
        /\b(echo|cd|pwd|ls|rm|cp|mv|cat|grep|find|chmod|chown|mkdir|rmdir|touch|exit|source)\b/,
        "type.identifier",
      ],

      // Variables like $VAR, $1, etc.
      [/\$\w+/, "variable"],

      // Strings
      [/"/, "string", "@string_double"],
      [/'/, "string", "@string_single"],

      // Operators
      [/@symbols/, "delimiter"],

      // Numbers
      [/\b\d+\b/, "number"],
    ],

    string_double: [
      [/[^\\"]+/, "string"],
      [/\\./, "string.escape"],
      [/"/, "string", "@pop"],
    ],

    string_single: [
      [/[^\\']+/, "string"],
      [/\\./, "string.escape"],
      [/'/, "string", "@pop"],
    ],
  },
};

monaco.languages.register({ id: "shell" });
monaco.languages.setLanguageConfiguration("shell", shell_conf);
monaco.languages.setMonarchTokensProvider("shell", shell_language);
