import { ConfigItem } from "@components/config/util";
import { useRead } from "@hooks";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";

export const NetworkModeSelector = ({
  selected,
  onSelect,
}: {
  selected: string | undefined;
  onSelect: (type: string) => void;
}) => {
  const networks = useRead("GetAvailableNetworks", {}).data;
  // const networks = useRead("GetDockerNetworks", {}).data?.forEach(network => network.)

  return (
    <ConfigItem label="Network Mode">
      <Select value={selected || undefined} onValueChange={onSelect}>
        <SelectTrigger className="max-w-[150px]">
          <SelectValue placeholder="Select Type" />
        </SelectTrigger>
        <SelectContent>
          {["Host", "Bridge", "None"].map((network) => (
            <SelectItem key={network} value={network.toLowerCase()}>
              {network}
            </SelectItem>
          ))}
          {networks?.networks.map((network) => (
            <SelectItem key={network.Id} value={network.Name ?? ""}>
              {network.Name}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </ConfigItem>
  );
};
