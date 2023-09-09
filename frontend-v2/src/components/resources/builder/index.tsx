import { useRead } from "@lib/hooks";
import { RequiredResourceComponents } from "@types";
import { Cloud, Bot, Factory } from "lucide-react";

const useBuilder = (id?: string) =>
  useRead("ListBuilders", {}).data?.find((d) => d.id === id);

export const Builder: RequiredResourceComponents = {
  Name: ({ id }) => <>{useBuilder(id)?.name}</>,
  Description: ({ id }) => <>{id}</>,
  Info: ({ id }) => (
    <>
      <div className="flex items-center gap-2">
        <Cloud className="w-4 h-4" />
        {useBuilder(id)?.info.provider}
      </div>
      <div className="flex items-center gap-2">
        <Bot className="w-4 h-4" />
        {useBuilder(id)?.info.instance_type ?? "N/A"}
      </div>
    </>
  ),
  Icon: () => <Factory className="w-4 h-4" />,
  Page: {},
  Actions: () => null,
  New: () => null,
};
