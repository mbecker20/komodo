export type Config = {
	mongoURL: string;
	registryURL: string;
};

export type SetConfig = (
	field: keyof Config,
	val: string | number | boolean
) => void;

export type Next = () => void;
