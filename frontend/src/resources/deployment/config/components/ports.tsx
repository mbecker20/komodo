import { ConfigItem, DoubleInput } from "@components/config/util";
import { Types } from "@monitor/client";

export const PortsConfig = ({
  ports,
  set,
}: {
  ports: Types.Conversion[];
  set: (input: Partial<Types.DeploymentConfig>) => void;
}) => (
  <ConfigItem label="Ports" className="items-start">
    <DoubleInput
      values={ports}
      leftval="local"
      leftpl="Local"
      rightval="container"
      rightpl="Container"
      addName="Port"
      onLeftChange={(local, i) => {
        ports[i].local = local;
        set({ ports: [...ports] });
      }}
      onRightChange={(container, i) => {
        ports[i].container = container;
        set({ ports: [...ports] });
      }}
      onAdd={() =>
        set({ ports: [...(ports ?? []), { container: "", local: "" }] })
      }
      onRemove={(idx) => set({ ports: [...ports.filter((_, i) => i !== idx)] })}
    />
  </ConfigItem>
);
