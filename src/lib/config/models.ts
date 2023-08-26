export type Config = {
	prompt_disable_package: boolean;
	custom_adb_path: string;
};

export const defaultConfig = {
	prompt_disable_package: true,
	custom_adb_path: ''
};
