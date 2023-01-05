import { Component } from "solid-js";
import { combineClasses } from "../../../../../util/helpers";
import Input from "../../../../shared/Input";
import Flex from "../../../../shared/layout/Flex";
import { useConfig } from "../Provider";

const PostImage: Component<{}> = (p) => {
  const { deployment, setDeployment, userCanUpdate } = useConfig();
  return (
    <Flex
      class={combineClasses("config-item shadow")}
      justifyContent="space-between"
    >
      <h1>post image</h1>
      <Input
        placeholder="post image"
        spellcheck={false}
        value={deployment.docker_run_args.post_image || ""}
        style={{ width: userCanUpdate() ? "16rem" : undefined }}
        onEdit={(post_image) =>
          setDeployment("docker_run_args", { post_image })
        }
        disabled={!userCanUpdate()}
      />
    </Flex>
  );
};

export default PostImage;
