export type Config = {
  monitorCore: {
		name: string;
		
	};
  mongo: {
    url: string;
    deploymentConfig?: {
      // if mongo container already exists locally running on docker and should be added as a monitor managed deployment
    };
    startConfig?: {
      // if this is attached, the cli will start container with this config and add
    };
  };
  registry: {
    exists: boolean; // if this is false, dont use a registry and builds will be disabled
    url?: string;
    deploymentConfig?: {};
    startConfig?: {};
  };
};

export type SetConfig = (
	domain: keyof Config,
	field: keyof Config[keyof Config],
	val: string | number | boolean
) => void;

export type Next = () => void;
