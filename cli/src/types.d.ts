export type Config = {
	useBuilds: boolean; // will set up the registry and enable docker build functionality
	mongoURL: string;
};

export type SetConfig = (
	field: keyof Config,
	val: string | number | boolean
) => void;
