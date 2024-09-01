import { DoubleInput } from "@components/config/util";
import { Types } from "@komodo/client";

export const VolumesConfig = ({
  volumes,
  set,
  disabled,
}: {
  volumes: Types.Conversion[];
  set: (input: Partial<Types.DeploymentConfig>) => void;
  disabled: boolean;
}) => (
  <div className="py-2 w-full flex justify-end">
    <DoubleInput
      disabled={disabled}
      inputClassName="w-[300px] 2xl:w-[400px] max-w-full"
      containerClassName="w-fit"
      values={volumes}
      leftval="local"
      leftpl="Local"
      rightval="container"
      rightpl="Container"
      onLeftChange={(local, i) => {
        volumes[i].local = local;
        set({ volumes: [...volumes] });
      }}
      onRightChange={(container, i) => {
        volumes[i].container = container;
        set({ volumes: [...volumes] });
      }}
      onRemove={(idx) =>
        set({ volumes: [...volumes.filter((_, i) => i !== idx)] })
      }
    />
  </div>
);
