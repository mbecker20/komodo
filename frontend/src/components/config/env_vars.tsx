import { ConfigItem, SecretSelector } from "@components/config/util";
import { useRead } from "@lib/hooks";
import { Types } from "@komodo/client";
import { MonacoEditor } from "@components/monaco";
import { useToast } from "@ui/use-toast";

export const EnvVars = ({
  env,
  set,
  disabled,
  server,
}: {
  env: string;
  set: (
    input: Partial<
      Types.DeploymentConfig | Types.StackConfig | Types.RepoConfig
    >
  ) => void;
  disabled: boolean;
  /// eg server id
  server?: string;
}) => {
  return (
    <ConfigItem className="flex-col gap-2 items-start">
      {!disabled && <Secrets server={server} />}
      <MonacoEditor
        value={env}
        onValueChange={(environment) => set({ environment })}
        language="yaml"
      />
    </ConfigItem>
  );
};

export const BuildArgs = ({
  type,
  args,
  set,
  disabled,
}: {
  type: "build" | "secret";
  args: string;
  set: (input: Partial<Types.BuildConfig>) => void;
  disabled: boolean;
}) => {
  const setArgs = (args: string) => set({ [`${type}_args`]: args });
  return (
    <ConfigItem className="flex-col gap-4 items-start">
      {!disabled && <Secrets />}
      <MonacoEditor value={args} onValueChange={setArgs} language="yaml" />
    </ConfigItem>
  );
};

const Secrets = ({
  server,
}: {
  /// eg server id
  server?: string;
}) => {
  if (server) return <SecretsWithServer server={server} />;
  return <SecretsNoServer />;
};

const SecretsNoServer = () => {
  const variables = useRead("ListVariables", {}).data ?? [];
  const secrets = useRead("ListSecrets", {}).data ?? [];
  return <SecretsView variables={variables} secrets={secrets} />;
};

const SecretsWithServer = ({
  server,
}: {
  /// eg server id
  server: string;
}) => {
  const variables = useRead("ListVariables", {}).data ?? [];
  const secrets =
    useRead("ListSecrets", { target: { type: "Server", id: server } }).data ??
    [];
  return <SecretsView variables={variables} secrets={secrets} />;
};

const SecretsView = ({
  variables,
  secrets,
}: {
  variables: Types.ListVariablesResponse;
  secrets: Types.ListSecretsResponse;
}) => {
  const { toast } = useToast();
  if (variables.length === 0 && secrets.length === 0) return;
  return (
    <div className="flex items-center gap-2">
      {variables.length > 0 && (
        <SecretSelector
          type="Variable"
          keys={variables.map((v) => v.name)}
          onSelect={(variable) => {
            if (!variable) return;
            navigator.clipboard.writeText("[[" + variable + "]]");
            toast({ title: "Copied selection" });
          }}
          disabled={false}
        />
      )}
      {secrets.length > 0 && (
        <SecretSelector
          type="Secret"
          keys={secrets}
          onSelect={(secret) => {
            if (!secret) return;
            navigator.clipboard.writeText("[[" + secret + "]]");
            toast({ title: "Copied selection" });
          }}
          disabled={false}
        />
      )}
    </div>
  );
};
