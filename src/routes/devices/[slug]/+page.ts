import { selectedDeviceIDStore } from '../../../stores.js';

export function load({ params }) {
	let deviceId = params.slug;

	selectedDeviceIDStore.set(deviceId);

	return {
		deviceId: deviceId
	};
}
