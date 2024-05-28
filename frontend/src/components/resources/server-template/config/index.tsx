import { useRead } from "@lib/hooks";
import { AwsServerTemplateConfig } from "./aws";
import { HetznerServerTemplateConfig } from "./hetzner";
import { Types } from "@monitor/client";

export const ServerTemplateConfig = ({ id }: { id: string }) => {
  const config = useRead("GetServerTemplate", { server_template: id }).data
    ?.config;
  const perms = useRead("GetPermissionLevel", {
    target: { type: "ServerTemplate", id },
  }).data;
  const global_disabled =
    useRead("GetCoreInfo", {}).data?.ui_write_disabled ?? false;
  const disabled = global_disabled || perms !== Types.PermissionLevel.Write;
  if (config?.type === "Aws") {
    return <AwsServerTemplateConfig id={id} disabled={disabled} />;
  } else if (config?.type === "Hetzner") {
    return <HetznerServerTemplateConfig id={id} disabled={disabled} />;
  }
};
