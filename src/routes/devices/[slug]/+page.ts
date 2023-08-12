import { error } from '@sveltejs/kit';
import { devicesWithUsersStore } from '../../../stores.js';

export function load({ params }) {
	let deviceID = params.slug;

	let activeDevices: string[] = [];

	devicesWithUsersStore.subscribe((devices) => {
		devices.forEach((val) => activeDevices.push(val.device.id));
	});

	if (!activeDevices.includes(deviceID)) {
		throw error(400, 'Invalid Device Id');
	}

	return {
		deviceID: deviceID
	};
}
