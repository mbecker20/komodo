import * as monaco from "monaco-editor";

export const dockerfile_conf: monaco.languages.LanguageConfiguration = {
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

export const dockerfile_language = <monaco.languages.IMonarchLanguage>{
  defaultToken: "",
  tokenPostfix: ".dockerfile",

  variable: /\${?[\w]+}?/,

  tokenizer: {
    root: [
      { include: "@whitespace" },
      { include: "@comment" },

      [/(ONBUILD)(\s+)/, ["keyword", ""]],
      [
        /(ENV)(\s+)([\w]+)/,
        ["keyword", "", { token: "variable", next: "@arguments" }],
      ],
      [
        /(FROM|MAINTAINER|RUN|EXPOSE|ENV|ADD|ARG|VOLUME|LABEL|USER|WORKDIR|COPY|CMD|STOPSIGNAL|SHELL|HEALTHCHECK|ENTRYPOINT)/,
        { token: "keyword", next: "@arguments" },
      ],
    ],

    arguments: [
      { include: "@whitespace" },
      { include: "@strings" },

      [
        /(@variable)/,
        {
          cases: {
            "@eos": { token: "variable", next: "@popall" },
            "@default": "variable",
          },
        },
      ],
      [
        /\\/,
        {
          cases: {
            "@eos": "",
            "@default": "",
          },
        },
      ],
      [
        /./,
        {
          cases: {
            "@eos": { token: "", next: "@popall" },
            "@default": "",
          },
        },
      ],
    ],

    // Deal with white space, including comments
    whitespace: [
      [
        /\s+/,
        {
          cases: {
            "@eos": { token: "", next: "@popall" },
            "@default": "",
          },
        },
      ],
    ],

    comment: [[/(^#.*$)/, "comment", "@popall"]],

    // Recognize strings, including those broken across lines with \ (but not without)
    strings: [
      [/\\'$/, "", "@popall"], // \' leaves @arguments at eol
      [/\\'/, ""], // \' is not a string
      [/'$/, "string", "@popall"],
      [/'/, "string", "@stringBody"],
      [/"$/, "string", "@popall"],
      [/"/, "string", "@dblStringBody"],
    ],
    stringBody: [
      [
        /[^\\\$']/,
        {
          cases: {
            "@eos": { token: "string", next: "@popall" },
            "@default": "string",
          },
        },
      ],

      [/\\./, "string.escape"],
      [/'$/, "string", "@popall"],
      [/'/, "string", "@pop"],
      [/(@variable)/, "variable"],

      [/\\$/, "string"],
      [/$/, "string", "@popall"],
    ],
    dblStringBody: [
      [
        /[^\\\$"]/,
        {
          cases: {
            "@eos": { token: "string", next: "@popall" },
            "@default": "string",
          },
        },
      ],

      [/\\./, "string.escape"],
      [/"$/, "string", "@popall"],
      [/"/, "string", "@pop"],
      [/(@variable)/, "variable"],

      [/\\$/, "string"],
      [/$/, "string", "@popall"],
    ],
  },
};

monaco.languages.register({ id: "dockerfile" });
monaco.languages.setLanguageConfiguration("dockerfile", dockerfile_conf);
monaco.languages.setMonarchTokensProvider("dockerfile", dockerfile_language);
