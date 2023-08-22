import type { User } from "$lib/users/models";

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