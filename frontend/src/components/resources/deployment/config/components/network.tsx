import { ConfigItem } from "@components/config/util";
import { useRead } from "@lib/hooks";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";

export const NetworkModeSelector = ({
  server_id,
  selected,
  onSelect,
}: {
  server_id: string | undefined;
  selected: string | undefined;
  onSelect: (type: string) => void;
}) => {
  const networks = useRead(
    "GetDockerNetworks",
    { server: server_id! },
    { enabled: !!server_id }
  ).data;

  return (
    <ConfigItem label="Network Mode">
      <Select value={selected || undefined} onValueChange={onSelect}>
        <SelectTrigger className="max-w-[150px]">
          <SelectValue placeholder="Select Type" />
        </SelectTrigger>
        <SelectContent>
          {networks?.map((network) => (
            <SelectItem key={network.Id} value={network.Name ?? ""} className="cursor-pointer">
              {network.Name}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </ConfigItem>
  );
};
