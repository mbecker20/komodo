import { ConfigItem } from "@components/config/util";
import { useRead } from "@lib/hooks";
import { env_to_text, text_to_env } from "@lib/utils";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { Textarea } from "@ui/textarea";
import { RefObject, createRef, useEffect, useState } from "react";

export const EnvVars = ({
  vars,
  set,
  disabled,
  server,
}: {
  vars: Types.EnvironmentVar[];
  set: (input: Partial<Types.DeploymentConfig>) => void;
  disabled: boolean;
  /// eg server id
  server?: string;
}) => {
  const ref = createRef<HTMLTextAreaElement>();
  const [env, setEnv] = useState(env_to_text(vars));
  useEffect(() => {
    !!env && set({ environment: text_to_env(env) });
  }, [env, set]);

  return (
    <ConfigItem className="flex-col gap-4 items-start">
      {!disabled && server && (
        <Secrets server={server} env={env} setEnv={setEnv} envRef={ref} />
      )}
      <Textarea
        ref={ref}
        className="min-h-[400px]"
        placeholder="VARIABLE=value"
        value={env}
        onChange={(e) => setEnv(e.target.value)}
        disabled={disabled}
      />
    </ConfigItem>
  );
};

const Secrets = ({
  env,
  setEnv,
  envRef,
  server,
}: {
  env?: string;
  setEnv: (env: string) => void;
  envRef: RefObject<HTMLTextAreaElement>;
  /// eg server id
  server: string;
}) => {
  const secrets = useRead("GetAvailableSecrets", { server }).data;
  const _env = env || "";
  return (
    secrets &&
    secrets.length > 0 && (
      <div className="flex gap-4 items-center">
        <div className="text-muted-foreground">secrets:</div>
        {secrets?.map((secret) => (
          <Button
            variant="secondary"
            key={secret}
            onClick={() =>
              setEnv(
                _env.slice(0, envRef.current?.selectionStart) +
                  `[[${secret}]]` +
                  _env.slice(envRef.current?.selectionStart, undefined)
              )
            }
          >
            {secret}
          </Button>
        ))}
      </div>
    )
  );
};
