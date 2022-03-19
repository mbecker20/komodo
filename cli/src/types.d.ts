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
    startConfig?: StartConfig;
  };
  registry?: {
    url: string;
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
  val: Config[keyof Config]
) => void;