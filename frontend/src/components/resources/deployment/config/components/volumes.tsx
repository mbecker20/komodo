import { ConfigItem, DoubleInput } from "@components/config/util";
import { Types } from "@monitor/client";

export const VolumesConfig = ({
  volumes,
  set,
  disabled,
}: {
  volumes: Types.Conversion[];
  set: (input: Partial<Types.DeploymentConfig>) => void;
  disabled: boolean;
}) => (
  <ConfigItem label="Volumes" className="items-start">
    <DoubleInput
      disabled={disabled}
      inputClassName="w-[300px] max-w-full"
      values={volumes}
      leftval="local"
      leftpl="Local"
      rightval="container"
      rightpl="Container"
      addName="Volume"
      onLeftChange={(local, i) => {
        volumes[i].local = local;
        set({ volumes: [...volumes] });
      }}
      onRightChange={(container, i) => {
        volumes[i].container = container;
        set({ volumes: [...volumes] });
      }}
      onAdd={() =>
        set({ volumes: [...(volumes ?? []), { container: "", local: "" }] })
      }
      onRemove={(idx) =>
        set({ volumes: [...volumes.filter((_, i) => i !== idx)] })
      }
    />
  </ConfigItem>
);
