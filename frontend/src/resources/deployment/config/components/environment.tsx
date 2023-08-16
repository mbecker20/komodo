import { DoubleInput } from "@components/config/util";
import { Types } from "@monitor/client";

export const EnvVars = ({
  vars,
  set,
}: {
  vars: Types.EnvironmentVar[];
  set: (input: Partial<Types.DeploymentConfig>) => void;
}) => (
  <DoubleInput
    values={vars}
    leftval="variable"
    leftpl="Variable"
    rightval="value"
    rightpl="Value"
    addName="Environment Varialbe"
    onLeftChange={(variable, i) => {
      vars[i].variable = variable;
      set({ environment: [...vars] });
    }}
    onRightChange={(value, i) => {
      vars[i].value = value;
      set({ environment: [...vars] });
    }}
    onAdd={() =>
      set({ environment: [...(vars ?? []), { variable: "", value: "" }] })
    }
    onRemove={(idx) =>
      set({ environment: [...vars.filter((_, i) => i !== idx)] })
    }
  />
);
