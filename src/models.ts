export type User = {
	id: string;
	name: string;
	device_id: string;
};

export type Device = {
	id: string;
	name: string;
	model: string;
	state: string;
};

export type DeviceWithUsers = {
	device: Device;
	users: [User];
};

export type Package = {
	name: string;
	ptype: string;
	state: string;
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
