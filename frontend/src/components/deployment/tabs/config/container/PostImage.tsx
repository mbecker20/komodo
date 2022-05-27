import { Component } from "solid-js";
import { useTheme } from "../../../../../state/ThemeProvider";
import { combineClasses } from "../../../../../util/helpers";
import Input from "../../../../util/Input";
import Flex from "../../../../util/layout/Flex";
import { useConfig } from "../Provider";

const PostImage: Component<{}> = (p) => {
  const { deployment, setDeployment, userCanUpdate } = useConfig();
  const { themeClass } = useTheme();
  return (
    <Flex
      class={combineClasses("config-item shadow", themeClass())}
      justifyContent="space-between"
    >
      <h1>post image</h1>
      <Input
        placeholder="post image"
        spellcheck={false}
        value={deployment.postImage || ""}
        style={{ width: userCanUpdate() && "16rem" }}
        onEdit={(value) => setDeployment("postImage", value)}
        disabled={!userCanUpdate()}
      />
    </Flex>
  );
};

export default PostImage;
