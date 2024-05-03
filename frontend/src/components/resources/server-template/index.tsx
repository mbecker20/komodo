import { useRead } from "@lib/hooks";
import { RequiredResourceComponents } from "@types";
import { DeleteResource, NewResource } from "../common";
import { Bot, Cloud } from "lucide-react";
import { ServerTemplateConfig } from "./config";

const useServerTemplate = (id?: string) =>
  useRead("ListServerTemplates", {}).data?.find((d) => d.id === id);

export const ServerTemplateComponents: RequiredResourceComponents = {
  Dashboard: () => {
    const count = useRead("ListServerTemplates", {}).data?.length;
    return <>{count}</>;
  },

  New: () => <NewResource type="ServerTemplate" />,

  Table: () => <div></div>,

  Name: ({ id }) => <>{useServerTemplate(id)?.name}</>,
  name: (id) => useServerTemplate(id)?.name,

  Icon: () => <></>,

  Status: {},

  Info: {
    Provider: ({ id }) => {
      const provider = useServerTemplate(id)?.info.provider;
      return (
        <div className="flex items-center gap-2">
          <Cloud className="w-4 h-4" />
          {provider}
        </div>
      );
    },
    InstanceType: ({ id }) => {
      const instanceType = useServerTemplate(id)?.info.instance_type;
      return (
        <div className="flex items-center gap-2">
          <Bot className="w-4 h-4" />
          {instanceType}
        </div>
      );
    },
  },

  Actions: {},

  Page: {},

  Config: ServerTemplateConfig,

  DangerZone: ({ id }) => <DeleteResource type="ServerTemplate" id={id} />,
};
