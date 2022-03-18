export type Config = {
	useBuilds: boolean; // will set up the registry and enable docker build functionality
	mongoURL: string;
	registryURL: string;
};

export type SetConfig = (
	field: keyof Config,
	val: string | number | boolean
) => void;

export type Next = () => void;
