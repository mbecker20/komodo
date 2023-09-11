import { useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { useState } from "react";
import {
  AccountSelector,
  ConfigInput,
  ConfigItem,
  ResourceSelector,
} from "@components/config/util";
import { ImageConfig } from "./components/image";
import { RestartModeSelector } from "./components/restart";
import { NetworkModeSelector } from "./components/network";
import { PortsConfig } from "./components/ports";
import { EnvVars } from "./components/environment";
import { VolumesConfig } from "./components/volumes";
import { ExtraArgs } from "./components/extra-args";
import { ConfigInner } from "@components/config";

export const ServerSelector = ({
  selected,
  set,
}: {
  selected: string | undefined;
  set: (input: Partial<Types.DeploymentConfig>) => void;
}) => (
  <ConfigItem label="Server">
    <ResourceSelector
      type="Server"
      selected={selected}
      onSelect={(server_id) => set({ server_id })}
    />
  </ConfigItem>
);

export const DeploymentConfig = ({ id }: { id: string }) => {
  const config = useRead("GetDeployment", { id }).data?.config;
  const [update, set] = useState<Partial<Types.DeploymentConfig>>({});
  const { mutate } = useWrite("UpdateDeployment");

  if (!config) return null;

  const show_ports = update.network
    ? update.network !== "host"
    : config.network
    ? config.network !== "host"
    : false;

  return (
    <ConfigInner
      config={config}
      update={update}
      set={set}
      onSave={() => mutate({ id, config: update })}
      components={{
        general: {
          general: {
            server_id: (value, set) => (
              <ServerSelector selected={value} set={set} />
            ),
          },
          container: {
            image: (value, set) => <ImageConfig image={value} set={set} />,
            docker_account: (value, set) => (
              <AccountSelector
                id={update.server_id ?? config.server_id}
                account_type="docker"
                type="Server"
                selected={value}
                onSelect={(docker_account) => set({ docker_account })}
              />
            ),
            restart: (value, set) => (
              <RestartModeSelector selected={value} set={set} />
            ),
            extra_args: (value, set) => (
              <ExtraArgs args={value ?? []} set={set} />
            ),
            process_args: (value, set) => (
              <ConfigInput
                label="Process Args"
                value={value}
                onChange={(process_args) => set({ process_args })}
              />
            ),
          },
          network: {
            network: (value, set) => (
              <NetworkModeSelector
                server_id={update.server_id ?? config.server_id}
                selected={value}
                onSelect={(network) => set({ network })}
              />
            ),
            ports: (value, set) =>
              show_ports && <PortsConfig ports={value ?? []} set={set} />,
          },
          volumes: {
            volumes: (v, set) => <VolumesConfig volumes={v ?? []} set={set} />,
          },
        },
        environment: {
          environment: {
            skip_secret_interp: true,
            environment: (vars, set) => <EnvVars vars={vars ?? []} set={set} />,
          },
        },
      }}
    />
  );
};