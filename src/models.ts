export type User = {
	id: string;
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