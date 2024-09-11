import React from "react";
import RemoteCodeFile from "./RemoteCodeFile";
import Tabs from "@theme/Tabs";
import TabItem from "@theme/TabItem";

export default function ComposeAndEnv({
  file_name,
}: {
  file_name: string;
}) {
  return (
    <Tabs>
      <TabItem value={file_name}>
        <RemoteCodeFile
          title={`https://github.com/mbecker20/komodo/blob/main/compose/${file_name}`}
          url={`https://raw.githubusercontent.com/mbecker20/komodo/1.14.2/compose/${file_name}`}
          language="yaml"
        />
      </TabItem>
      <TabItem value="compose.env">
        <RemoteCodeFile
          title="https://github.com/mbecker20/komodo/blob/main/compose/compose.env"
          url="https://raw.githubusercontent.com/mbecker20/komodo/1.14.2/compose/compose.env"
          language="bash"
        />
      </TabItem>
    </Tabs>
  );
}
