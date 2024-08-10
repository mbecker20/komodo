import { ConfigItem, SecretSelector } from "@components/config/util";
import { useRead } from "@lib/hooks";
import { Types } from "@monitor/client";
import { Textarea } from "@ui/textarea";
import { RefObject, createRef } from "react";

export const EnvVars = ({
  env,
  set,
  disabled,
  server,
}: {
  env: string;
  set: (input: Partial<Types.DeploymentConfig>) => void;
  disabled: boolean;
  /// eg server id
  server?: string;
}) => {
  const ref = createRef<HTMLTextAreaElement>();
  const setEnv = (environment: string) => set({ environment });

  return (
    <ConfigItem className="flex-col gap-2 items-start">
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
        spellCheck={false}
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
  const variables = useRead("ListVariables", {}).data ?? [];
  const secrets =
    useRead("ListSecrets", { target: { type: "Server", id: server } }).data ??
    [];

  const _env = env || "";

  if (variables.length === 0 && secrets.length === 0) return;

  return (
    <div className="flex items-center gap-2">
      {variables.length > 0 && (
        <SecretSelector
          type="Variable"
          keys={variables.map((v) => v.name)}
          onSelect={(variable) =>
            setEnv(
              _env.slice(0, envRef.current?.selectionStart) +
                `[[${variable}]]` +
                _env.slice(envRef.current?.selectionStart, undefined)
            )
          }
          disabled={false}
        />
      )}
      {secrets.length > 0 && (
        <SecretSelector
          type="Secret"
          keys={secrets}
          onSelect={(secret) =>
            setEnv(
              _env.slice(0, envRef.current?.selectionStart) +
                `[[${secret}]]` +
                _env.slice(envRef.current?.selectionStart, undefined)
            )
          }
          disabled={false}
        />
      )}
    </div>
  );
};
