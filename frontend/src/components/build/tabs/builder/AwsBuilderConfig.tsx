import { Component, Show } from "solid-js";
import { useAppState } from "../../../../state/StateProvider";
import Input from "../../../shared/Input";
import Flex from "../../../shared/layout/Flex";
import Selector from "../../../shared/menu/Selector";
import { useConfig } from "../Provider";

const AwsBuilderConfig: Component<{}> = (p) => {
  const { build } = useConfig();
  return (
    <>
      <Ami />
      <InstanceType />
      <VolumeSize />
      <Show when={!build.updated}>
        <div style={{ height: "4rem" }} />
      </Show>
    </>
  );
};

const Ami: Component = () => {
  const { aws_builder_config } = useAppState();
  const { build, setBuild, userCanUpdate } = useConfig();
  const get_ami_id = () => {
    if (build.aws_config?.ami_id) {
      return build.aws_config.ami_id;
    } else {
      return aws_builder_config()?.default_ami_id || "unknown";
    }
  };
  const get_ami_name = (ami_id: string) => {
    if (aws_builder_config() === undefined || ami_id === "unknown")
      return "unknown";
    return (
      aws_builder_config()!.available_ami_accounts![ami_id]?.name || "unknown"
    );
  };
  const ami_ids = () => {
    if (aws_builder_config() === undefined) return [];
    return Object.keys(aws_builder_config()!.available_ami_accounts!);
  };
  return (
    <Flex
      class="config-item shadow"
      alignItems="center"
      justifyContent="space-between"
    >
      <h1>ami</h1>
      <Selector
        targetClass="blue"
        selected={get_ami_id()}
        items={ami_ids()}
        onSelect={(ami_id) => setBuild("aws_config", "ami_id", ami_id)}
        itemMap={get_ami_name}
        position="bottom right"
        disabled={!userCanUpdate()}
        useSearch
      />
    </Flex>
  );
};

const InstanceType: Component = () => {
  const { aws_builder_config } = useAppState();
  const { build, setBuild, userCanUpdate } = useConfig();
  return (
    <Flex
      class="config-item shadow"
      alignItems="center"
      justifyContent="space-between"
    >
      <h1>instance type</h1>
      <Input
        placeholder={aws_builder_config()?.default_instance_type}
        value={build.aws_config?.instance_type}
        onEdit={(instance_type) =>
          setBuild("aws_config", "instance_type", instance_type)
        }
        disabled={!userCanUpdate()}
      />
    </Flex>
  );
};

const VolumeSize: Component = () => {
  const { aws_builder_config } = useAppState();
  const { build, setBuild, userCanUpdate } = useConfig();
  return (
    <Flex
      class="config-item shadow"
      alignItems="center"
      justifyContent="space-between"
    >
      <h1>volume size</h1>
      <Flex gap="0.25rem" alignItems="center">
        <Input
          style={{ width: "4rem" }}
          placeholder={aws_builder_config()?.default_volume_gb?.toString()}
          value={
            build.aws_config?.volume_gb
              ? build.aws_config.volume_gb.toString()
              : ""
          }
          onEdit={(volume_size) =>
            setBuild("aws_config", "volume_gb", Number(volume_size))
          }
          disabled={!userCanUpdate()}
        />
        GB
      </Flex>
    </Flex>
  );
};

export default AwsBuilderConfig;
