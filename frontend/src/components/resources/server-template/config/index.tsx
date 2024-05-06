import { useRead } from "@lib/hooks";
import { AwsServerTemplateConfig } from "./aws";

export const ServerTemplateConfig = ({ id }: { id: string }) => {
  const config = useRead("GetServerTemplate", { server_template: id }).data
    ?.config;
  if (config?.type === "Aws") return <AwsServerTemplateConfig id={id} />;
};