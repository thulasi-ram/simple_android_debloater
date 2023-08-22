export type Package = {
	name: string;
	ptype: string;
	state: string;
	package_prefix: string;
};

export type DeviceUserPackages = {
	device_id: string;
	user_id: string;
	packages: Package[];
};

export type DeviceUserPackage = {
	device_id: string;
	user_id: string;
	package: Package;
};


export type Config = {
	prompt_disable_package: boolean;
}