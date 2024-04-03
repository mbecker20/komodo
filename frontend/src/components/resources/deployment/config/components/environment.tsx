import { ConfigItem } from "@components/config/util";
import { useRead } from "@lib/hooks";
import { env_to_text, text_to_env } from "@lib/utils";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { Textarea } from "@ui/textarea";
import { useEffect, useState } from "react";

export const EnvVars = ({
  vars,
  set,
  server,
}: {
  vars: Types.EnvironmentVar[];
  set: (input: Partial<Types.DeploymentConfig>) => void;
  /// eg server id
  server?: string;
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
      {server && <Secrets server={server} />}
      <Textarea
        className="min-h-[400px]"
        placeholder="VARIABLE=value"
        value={env}
        onChange={(e) => setEnv(e.target.value)}
      />
    </ConfigItem>
  );
};

const Secrets = ({
  // vars,
  // set,
  server,
}: {
  // vars: Types.EnvironmentVar[];
  // set: (input: Partial<Types.DeploymentConfig>) => void;
  /// eg server id
  server: string;
}) => {
  const secrets = useRead("GetAvailableSecrets", { server }).data;
  return (
    <div className="w-full flex gap-4 justify-end">
      {secrets?.map((secret) => (
        <Button key={secret}>{secret}</Button>
      ))}
    </div>
  );
};
