import * as monaco from "monaco-editor";

export async function init_monaco() {
  const promises = ["lib", "responses", "types"].map((file) =>
    Promise.all(
      [".js", ".d.ts"].map((extension) =>
        fetch(`/client/${file}${extension}`)
          .then((res) => res.text())
          .then((dts) =>
            monaco.languages.typescript.typescriptDefaults.addExtraLib(
              dts,
              `file:///client/${file}${extension}`
            )
          )
      )
    )
  );
  await Promise.all(promises);

  fetch(`/index.d.ts`)
    .then((res) => res.text())
    .then((dts) =>
      monaco.languages.typescript.typescriptDefaults.addExtraLib(
        dts,
        `file:///index.d.ts`
      )
    );

  monaco.languages.typescript.typescriptDefaults.setCompilerOptions({
    module: monaco.languages.typescript.ModuleKind.ESNext,
    target: monaco.languages.typescript.ScriptTarget.ESNext,
    allowNonTsExtensions: true,
    moduleResolution: monaco.languages.typescript.ModuleResolutionKind.NodeJs,
    typeRoots: ["index.d.ts"],
  });

  monaco.languages.typescript.typescriptDefaults.setDiagnosticsOptions({
    diagnosticCodesToIgnore: [1375],
  });
}
