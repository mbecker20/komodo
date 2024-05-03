import { ConfigItem } from "@components/config/util";
import { Types } from "@monitor/client";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";
import { object_keys } from "@lib/utils";

const format_mode = (m: string) => m.split("-").join(" ");

export const RestartModeSelector = ({
  selected,
  set,
  disabled,
}: {
  selected: Types.RestartMode | undefined;
  set: (input: Partial<Types.DeploymentConfig>) => void;
  disabled: boolean;
}) => (
  <ConfigItem label="Restart Mode">
    <Select
      value={selected || undefined}
      onValueChange={(restart: Types.RestartMode) => set({ restart })}
      disabled={disabled}
    >
      <SelectTrigger className="max-w-[150px] capitalize" disabled={disabled}>
        <SelectValue placeholder="Select Type" />
      </SelectTrigger>
      <SelectContent>
        {object_keys(Types.RestartMode).map((mode) => (
          <SelectItem
            key={mode}
            value={Types.RestartMode[mode]}
            className="capitalize cursor-pointer"
          >
            {mode === "NoRestart"
              ? "Don't Restart"
              : format_mode(Types.RestartMode[mode])}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  </ConfigItem>
);
