import { ConfigItem } from "@components/config/util";
import { Types } from "@monitor/client";
import { RestartMode } from "@monitor/client/dist/types";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";
import { keys } from "@util/helpers";

export const RestartModeSelector = ({
  selected,
  set,
}: {
  selected: Types.RestartMode | undefined;
  set: (input: Partial<Types.DeploymentConfig>) => void;
}) => (
  <ConfigItem label="Restart Mode">
    <Select
      value={selected || undefined}
      onValueChange={(restart: Types.RestartMode) => set({ restart })}
    >
      <SelectTrigger className="max-w-[150px]">
        <SelectValue placeholder="Select Type" />
      </SelectTrigger>
      <SelectContent>
        {keys(RestartMode).map((mode) => (
          <SelectItem value={mode}>{mode}</SelectItem>
        ))}
      </SelectContent>
    </Select>
  </ConfigItem>
);
