import { fill_color_class_by_intention } from "@lib/color";
import { useRead } from "@lib/hooks";
import { Hammer } from "lucide-react";

export const IconStrictId = ({ id }: { id: string }) => {
  const building = useRead("GetBuildActionState", { build: id }).data?.building;
  const className = building
    ? "w-4 h-4 animate-spin " + fill_color_class_by_intention("Good")
    : "w-4 h-4";
  return <Hammer className={className} />;
};
