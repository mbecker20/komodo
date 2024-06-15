import { ConfigItem } from "@components/config/util";
import { useRead } from "@lib/hooks";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@ui/tabs";
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
  const { variables, secrets: core_secrets } = useRead("ListVariables", {})
    .data ?? {
    variables: [],
    secrets: [],
  };
  const periphery_secrets =
    useRead("GetAvailableSecrets", { server }).data || [];

  // Get unique list of secrets between core and periphery
  const secrets = [...new Set([...core_secrets, ...periphery_secrets])];

  const _env = env || "";

  if (variables.length === 0 && secrets.length === 0) return;

  if (variables.length === 0) {
    // ONLY SECRETS
    return (
      <div className="flex flex-col gap-2 w-full">
        <h2 className="text-muted-foreground">Secrets</h2>
        <div className="flex gap-4 items-center flex-wrap w-full">
          {secrets.map((secret) => (
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
      </div>
    );
  }

  if (secrets.length === 0) {
    // ONLY VARIABLES
    return (
      <div className="flex flex-col gap-2 w-full">
        <h2 className="text-muted-foreground">Variables</h2>
        <div className="flex gap-4 items-center flex-wrap w-full">
          {variables.map(({ name }) => (
            <Button
              variant="secondary"
              key={name}
              onClick={() =>
                setEnv(
                  _env.slice(0, envRef.current?.selectionStart) +
                    `[[${name}]]` +
                    _env.slice(envRef.current?.selectionStart, undefined)
                )
              }
            >
              {name}
            </Button>
          ))}
        </div>
      </div>
    );
  }

  return (
    <Tabs className="w-full" defaultValue="Variables">
      <TabsList>
        <TabsTrigger value="Variables">Variables</TabsTrigger>
        <TabsTrigger value="Secrets">Secrets</TabsTrigger>
      </TabsList>
      <TabsContent value="Variables">
        <div className="flex gap-4 items-center w-full flex-wrap pt-1">
          {variables.map(({ name }) => (
            <Button
              variant="secondary"
              key={name}
              onClick={() =>
                setEnv(
                  _env.slice(0, envRef.current?.selectionStart) +
                    `[[${name}]]` +
                    _env.slice(envRef.current?.selectionStart, undefined)
                )
              }
            >
              {name}
            </Button>
          ))}
        </div>
      </TabsContent>
      <TabsContent value="Secrets">
        <div className="flex gap-4 items-center w-full flex-wrap pt-1">
          {secrets.map((secret) => (
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
      </TabsContent>
    </Tabs>
  );
};
