export type Config = {
  monitorCore?: {
    name: string;
    secretVolume: string; //  to mount secrets.json into the container
    hostNetwork?: boolean;
    port?: number;
  };
  monitorPeriphery?: {
    name: string;
    hostNetwork?: boolean;
    port?: number;
  };
  mongo?: {
    url: string;
    deploymentConfig?: {
      // if mongo container already exists locally running on docker and should be added as a monitor managed deployment
    };
    startConfig?: StartConfig;
  };
  registry?: {
    exists: boolean; // if this is false, dont use a registry and builds will be disabled
    url?: string;
    deploymentConfig?: {};
    startConfig?: StartConfig;
  };
};

export type StartConfig = {
  // if this is attached, the cli will start container with this config and add
  name: string;
  port: number;
  volume: string | false;
  restart: string;
};

export type SetConfig = (
  field: keyof Config,
  val: string | number | boolean
) => void;

export type Next = () => void;
