import { ConfigItem } from "@components/config/util";
import { Types } from "@monitor/client";
import { Badge } from "@ui/badge";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
} from "@ui/select";
import { MinusCircle } from "lucide-react";

const ALERT_TYPES: Types.AlertData["type"][] = [
  "ServerUnreachable",
  "ServerCpu",
  "ServerMem",
  "ServerDisk",
  "ContainerStateChange",
  "AwsBuilderTerminationFailed",
];

export const AlertTypeConfig = ({
  alert_types,
  set,
  disabled,
}: {
  alert_types: Types.AlertData["type"][];
  set: (alert_types: Types.AlertData["type"][]) => void;
  disabled: boolean;
}) => {
  const at = ALERT_TYPES.filter(
    (alert_type) => !alert_types.includes(alert_type)
  );
  return (
    <ConfigItem label="Alert Types">
      <div className="flex items-center gap-4">
        <div className="flex items-center gap-2">
          {alert_types.map((type) => (
            <Badge
              variant="secondary"
              className="text-sm flex items-center gap-2 cursor-pointer"
              onClick={() => {
                if (disabled) return;
                set(alert_types.filter((t) => t !== type));
              }}
            >
              {type}
              {!disabled && <MinusCircle className="w-3 h-3" />}
            </Badge>
          ))}
        </div>
        {at.length ? (
          <Select
            value={undefined}
            onValueChange={(type: Types.AlertData["type"]) => {
              set([...alert_types, type]);
            }}
            disabled={disabled}
          >
            <SelectTrigger className="w-[150px]">
              <div className="pr-2">Add Filter</div>
            </SelectTrigger>
            <SelectContent align="end">
              {at.map((alert_type) => (
                <SelectItem key={alert_type} value={alert_type}>
                  {alert_type}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        ) : undefined}
      </div>
    </ConfigItem>
  );
};
