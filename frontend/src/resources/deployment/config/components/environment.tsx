import { ConfigItem } from "@components/config/util";
import { Types } from "@monitor/client";
import { Textarea } from "@ui/textarea";
import { parseDotEnvToEnvVars, parseEnvVarseToDotEnv } from "@util/helpers";
import { useEffect, useState } from "react";

export const EnvVars = ({
  vars,
  set,
}: {
  vars: Types.EnvironmentVar[];
  set: (input: Partial<Types.DeploymentConfig>) => void;
}) => {
  const [env, setEnv] = useState(parseEnvVarseToDotEnv(vars));
  useEffect(() => {
    !!env && set({ environment: parseDotEnvToEnvVars(env) });
  }, [env, set]);

  return (
    <ConfigItem
      label="Environment Variables"
      className="flex-col gap-4 items-start"
    >
      <Textarea
        placeholder="VARIABLE=value"
        value={env}
        onChange={(e) => setEnv(e.target.value)}
      />
    </ConfigItem>
  );
};
