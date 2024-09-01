import { DoubleInput } from "@components/config/util";
import { Types } from "@komodo/client";

export const PortsConfig = ({
  ports,
  set,
  disabled,
}: {
  ports: Types.Conversion[];
  set: (input: Partial<Types.DeploymentConfig>) => void;
  disabled: boolean;
}) => (
  <div className="py-2 w-full flex justify-end">
    <DoubleInput
      disabled={disabled}
      inputClassName="w-[200px] max-w-full"
      containerClassName="w-fit"
      values={ports}
      leftval="local"
      leftpl="Local"
      rightval="container"
      rightpl="Container"
      onLeftChange={(local, i) => {
        ports[i].local = local;
        set({ ports: [...ports] });
      }}
      onRightChange={(container, i) => {
        ports[i].container = container;
        set({ ports: [...ports] });
      }}
      onRemove={(idx) => set({ ports: [...ports.filter((_, i) => i !== idx)] })}
    />
  </div>
);
