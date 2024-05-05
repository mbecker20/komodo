import { useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { useState } from "react";
import { AccountSelector, ConfigItem } from "@components/config/util";
import { ImageConfig } from "./components/image";
import { RestartModeSelector } from "./components/restart";
import { NetworkModeSelector } from "./components/network";
import { PortsConfig } from "./components/ports";
import { EnvVars } from "./components/environment";
import { VolumesConfig } from "./components/volumes";
import { ExtraArgs } from "./components/extra-args";
import { Config } from "@components/config";
import {
  DefaultTerminationSignal,
  TermSignalLabels,
  TerminationTimeout,
} from "./components/term-signal";
import { LabelsConfig, ResourceSelector } from "@components/resources/common";
import { TextUpdateMenu } from "@components/util";

export const ServerSelector = ({
  selected,
  set,
  disabled,
}: {
  selected: string | undefined;
  set: (input: Partial<Types.DeploymentConfig>) => void;
  disabled: boolean;
}) => (
  <ConfigItem label="Server">
    <ResourceSelector
      type="Server"
      selected={selected}
      onSelect={(server_id) => set({ server_id })}
      disabled={disabled}
    />
  </ConfigItem>
);

export const DeploymentConfig = ({ id }: { id: string }) => {
  const perms = useRead("GetPermissionLevel", {
    target: { type: "Deployment", id },
  }).data;
  const config = useRead("GetDeployment", { deployment: id }).data?.config;
  const [update, set] = useState<Partial<Types.DeploymentConfig>>({});
  const { mutateAsync } = useWrite("UpdateDeployment");

  if (!config) return null;

  const show_ports = update.network
    ? update.network !== "host"
    : config.network
    ? config.network !== "host"
    : false;

  const disabled = perms !== Types.PermissionLevel.Write;

  return (
    <Config
      disabled={disabled}
      config={config}
      update={update}
      set={set}
      onSave={async () => {
        await mutateAsync({ id, config: update });
      }}
      components={{
        general: {
          "": {
            server_id: (value, set) => (
              <ServerSelector selected={value} set={set} disabled={disabled} />
            ),
          },
          container: {
            image: (value, set) => (
              <ImageConfig image={value} set={set} disabled={disabled} />
            ),
            docker_account: (value, set) => (
              <AccountSelector
                id={update.server_id ?? config.server_id}
                account_type="docker"
                type="Server"
                selected={value}
                onSelect={(docker_account) => set({ docker_account })}
                disabled={disabled}
                placeholder={
                  (update.image?.type || config.image?.type) === "Build"
                    ? "Same as build"
                    : "None"
                }
              />
            ),
            restart: (value, set) => (
              <RestartModeSelector
                selected={value}
                set={set}
                disabled={disabled}
              />
            ),
            process_args: (value, set) => (
              <ConfigItem label="Process Args">
                <TextUpdateMenu
                  title="Update Process Args"
                  placeholder="Set Process Args"
                  value={value}
                  onUpdate={(process_args) => set({ process_args })}
                  triggerClassName="min-w-[300px] max-w-[400px]"
                />
              </ConfigItem>
            ),
            network: (value, set) => (
              <NetworkModeSelector
                server_id={update.server_id ?? config.server_id}
                selected={value}
                onSelect={(network) => set({ network })}
                disabled={disabled}
              />
            ),
            ports: (value, set) =>
              show_ports && (
                <PortsConfig
                  ports={value ?? []}
                  set={set}
                  disabled={disabled}
                />
              ),
            volumes: (v, set) => (
              <VolumesConfig volumes={v ?? []} set={set} disabled={disabled} />
            ),
            labels: (l, set) => (
              <LabelsConfig labels={l ?? []} set={set} disabled={disabled} />
            ),
            extra_args: (value, set) => (
              <ExtraArgs args={value ?? []} set={set} disabled={disabled} />
            ),
          },
          settings: {
            send_alerts: true,
            redeploy_on_build:
              (update.image?.type || config.image?.type) === "Build",
          },
        },
        environment: {
          environment: {
            environment: (vars, set) => (
              <EnvVars
                vars={vars ?? []}
                set={set}
                server={update.server_id || config.server_id}
                disabled={disabled}
              />
            ),
            skip_secret_interp: true,
          },
        },
        termination: {
          termination: {
            termination_signal: (value, set) => (
              <DefaultTerminationSignal
                arg={value}
                set={set}
                disabled={disabled}
              />
            ),
            termination_timeout: (value, set) => (
              <TerminationTimeout arg={value} set={set} disabled={disabled} />
            ),
            term_signal_labels: (value, set) => (
              <TermSignalLabels args={value} set={set} disabled={disabled} />
            ),
          },
        },
      }}
    />
  );
};
