import { useEffect, useState } from "react";
import monaco from "monaco-editor";
import { DiffEditor, Editor } from "@monaco-editor/react";
import { useTheme } from "@ui/theme";
import { cn } from "@lib/utils";

const MIN_EDITOR_HEIGHT = 56;
// const MAX_EDITOR_HEIGHT = 500;

export const MonacoEditor = ({
  value,
  onValueChange,
  language,
  readOnly,
}: {
  value: string | undefined;
  onValueChange?: (value: string) => void;
  language: "yaml" | "toml" | "json" | "shell" | "key_value" | undefined;
  readOnly?: boolean;
}) => {
  const [editor, setEditor] =
    useState<monaco.editor.IStandaloneCodeEditor | null>(null);

  const line_count = value?.split(/\r\n|\r|\n/).length ?? 0;

  useEffect(() => {
    if (!editor) return;
    const contentHeight = line_count * 18 + 30;
    const node = editor.getContainerDomNode();
    node.style.height = `${Math.max(
      Math.ceil(contentHeight),
      MIN_EDITOR_HEIGHT
    )}px`;
    // node.style.height = `${Math.max(
    //   Math.min(Math.ceil(contentHeight), MAX_EDITOR_HEIGHT),
    //   MIN_EDITOR_HEIGHT
    // )}px`;
  }, [editor, line_count]);

  const { theme: _theme } = useTheme();
  const theme =
    _theme === "system"
      ? window.matchMedia("(prefers-color-scheme: dark)").matches
        ? "dark"
        : "light"
      : _theme;

  const options: monaco.editor.IStandaloneEditorConstructionOptions = {
    minimap: { enabled: false },
    scrollbar: { alwaysConsumeMouseWheel: false },
    scrollBeyondLastLine: false,
    folding: false,
    automaticLayout: true,
    renderValidationDecorations: "on",
    renderLineHighlightOnlyWhenFocus: true,
    readOnly,
    tabSize: 2,
    detectIndentation: true,
    padding: {
      top: 15,
    },
  };

  return (
    <div className="mx-2 my-1 w-full">
      <Editor
        language={language}
        value={value}
        theme={theme}
        options={options}
        onChange={(v) => onValueChange?.(v ?? "")}
        onMount={(editor) => setEditor(editor)}
      />
    </div>
  );
};

const MIN_DIFF_HEIGHT = 100;
const MAX_DIFF_HEIGHT = 400;

export const MonacoDiffEditor = ({
  original,
  modified,
  onModifiedValueChange,
  language,
  readOnly,
  containerClassName,
  hideUnchangedRegions = true,
}: {
  original: string | undefined;
  modified: string | undefined;
  onModifiedValueChange?: (value: string) => void;
  language: "yaml" | "toml" | "json" | "shell" | "key_value" | undefined;
  readOnly?: boolean;
  containerClassName?: string;
  hideUnchangedRegions?: boolean;
}) => {
  const [editor, setEditor] =
    useState<monaco.editor.IStandaloneDiffEditor | null>(null);

  const original_line_count = original?.split(/\r\n|\r|\n/).length ?? 0;
  const modified_line_count = modified?.split(/\r\n|\r|\n/).length ?? 0;
  const line_count = Math.max(original_line_count, modified_line_count);

  useEffect(() => {
    if (!editor) return;
    const contentHeight = line_count * 18 + 30;
    const node = editor.getContainerDomNode();
    node.style.height = `${Math.max(
      Math.min(Math.ceil(contentHeight), MAX_DIFF_HEIGHT),
      MIN_DIFF_HEIGHT
    )}px`;
  }, [editor, line_count]);

  const { theme: _theme } = useTheme();
  const theme =
    _theme === "system"
      ? window.matchMedia("(prefers-color-scheme: dark)").matches
        ? "dark"
        : "light"
      : _theme;

  const options: monaco.editor.IStandaloneDiffEditorConstructionOptions = {
    minimap: { enabled: true },
    scrollbar: { alwaysConsumeMouseWheel: false },
    scrollBeyondLastLine: false,
    hideUnchangedRegions: { enabled: hideUnchangedRegions },
    folding: false,
    automaticLayout: true,
    renderValidationDecorations: "on",
    renderLineHighlightOnlyWhenFocus: true,
    readOnly,
    padding: {
      top: 15,
    },
  };

  return (
    <div className={cn("mx-2 my-1", containerClassName)}>
      <DiffEditor
        language={language}
        original={original}
        modified={modified}
        theme={theme}
        options={options}
        onMount={(editor) => {
          const modifiedEditor = editor.getModifiedEditor();
          modifiedEditor.onDidChangeModelContent((_) => {
            onModifiedValueChange?.(modifiedEditor.getValue());
          });
          setEditor(editor);
        }}
      />
    </div>
  );
};
