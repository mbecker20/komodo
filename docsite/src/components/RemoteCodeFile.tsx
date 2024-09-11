import React, { useEffect, useState } from "react";
import CodeBlock from "@theme/CodeBlock";

async function fetch_text_set(url: string, set: (text: string) => void) {
  const res = await fetch(url);
  const text = await res.text();
  set(text);
}

export default function RemoteCodeFile({
  url,
  language,
  title,
}: {
  url: string;
  language?: string;
  title?: string;
}) {
  const [file, setFile] = useState("");
  useEffect(() => {
    fetch_text_set(url, setFile);
  }, []);
  return (
    <CodeBlock title={title ?? url} language={language} showLineNumbers>
      {file}
    </CodeBlock>
  );
}
