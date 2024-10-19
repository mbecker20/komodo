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

import { loader } from "@monaco-editor/react";
loader.config({ monaco });

// Load the themes
import "./theme";
// Load the parsers
import "./yaml";
import "./toml";
import "./key_value";
import "./shell";
import "./dockerfile";
import "./rust";

export async function init_monaco() {
  const promises = ["lib", "responses", "types"].map((file) =>
    fetch(`/client/${file}.d.ts`)
      .then((res) => res.text())
      .then((dts) =>
        monaco.languages.typescript.typescriptDefaults.addExtraLib(
          dts,
          `file:///node_modules/@types/komodo_client/${
            file === "lib" ? "index" : file
          }.d.ts`
        )
      )
  );
  await Promise.all(promises);

  fetch(`/deno.d.ts`)
    .then((res) => res.text())
    .then((dts) =>
      monaco.languages.typescript.typescriptDefaults.addExtraLib(
        dts,
        `file:///node_modules/@types/deno/index.d.ts`
      )
    );

  monaco.languages.typescript.typescriptDefaults.setCompilerOptions({
    target: monaco.languages.typescript.ScriptTarget.ESNext,
    allowNonTsExtensions: true,
    moduleResolution: monaco.languages.typescript.ModuleResolutionKind.NodeJs,
    module: monaco.languages.typescript.ModuleKind.ESNext,
    typeRoots: ["node_modules/@types/deno/index.d.ts"],
    paths: {
      "npm:komodo_client": [
        "file:///node_modules/@types/komodo_client/index.d.ts",
      ],
    },
  });
}
