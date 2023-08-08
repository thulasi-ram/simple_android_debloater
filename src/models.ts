export type User = {
	id: string;
	name: string;
};

export type Device = {
	id: string;
	name: string;
	model: string;
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
