export type Config = {
  core?: CoreConfig;
  periphery?: PeripheryConfig;
  mongo?: {
    url: string;
    startConfig?: StartConfig;
  };
  registry?: {
    url: string;
    startConfig?: StartConfig;
  };
};

export type CoreConfig = {
  name: string;
  secretVolume: string; //  to mount secrets.json into the container
  hostNetwork: boolean;
  port: number;
};

export type PeripheryConfig = {
  name: string;
  hostNetwork: boolean;
  port: number;
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