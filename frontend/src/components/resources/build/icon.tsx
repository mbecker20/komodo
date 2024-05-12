import { fill_color_class_by_intention } from "@lib/color";
import { useRead } from "@lib/hooks";
import { Hammer } from "lucide-react";

export const IconStrictId = ({ id, size }: { id: string; size: number }) => {
  const status = useRead("ListBuilds", {}).data?.find(b => b.id === id)?.info.status;
  
  return <Hammer className={className} />;
};
