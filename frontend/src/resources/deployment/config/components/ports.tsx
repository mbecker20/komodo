import { DoubleInput } from "@components/config/util";
import { Types } from "@monitor/client";

export const PortsConfig = ({
  ports,
  set,
}: {
  ports: Types.Conversion[];
  set: (input: Partial<Types.DeploymentConfig>) => void;
}) => (
  <DoubleInput
    values={ports}
    leftval="container"
    rightval="local"
    onLeftChange={(container, i) => {
      ports[i].container = container;
      set({ ports: [...ports] });
    }}
    onRightChange={(local, i) => {
      ports[i].local = local;
      set({ ports: [...ports] });
    }}
    onAdd={() =>
      set({ ports: [...(ports ?? []), { container: "", local: "" }] })
    }
    onRemove={(idx) => set({ ports: [...ports.filter((_, i) => i !== idx)] })}
  />
);
