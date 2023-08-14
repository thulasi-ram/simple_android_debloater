import { selectedDeviceIDStore } from '../../../stores.js';

export function load({ params }) {
	let deviceID = params.slug;

	selectedDeviceIDStore.set(deviceID);

	return {
		deviceID: deviceID
	};
}
