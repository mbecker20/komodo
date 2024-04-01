import { ConfigItem } from "@components/config/util";
import { env_to_text, text_to_env } from "@lib/utils";
import { Types } from "@monitor/client";
import { Textarea } from "@ui/textarea";
import { useEffect, useState } from "react";

export const EnvVars = ({
  vars,
  set,
}: {
  vars: Types.EnvironmentVar[];
  set: (input: Partial<Types.DeploymentConfig>) => void;
}) => {
  const [env, setEnv] = useState(env_to_text(vars));
  useEffect(() => {
    !!env && set({ environment: text_to_env(env) });
  }, [env, set]);

  return (
    <ConfigItem
      label="Environment Variables"
      className="flex-col gap-4 items-start"
    >
      <Textarea
        className="min-h-[400px]"
        placeholder="VARIABLE=value"
        value={env}
        onChange={(e) => setEnv(e.target.value)}
      />
    </ConfigItem>
  );
};
