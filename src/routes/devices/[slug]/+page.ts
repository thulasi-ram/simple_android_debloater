import { selectedDeviceIDStore, selectedDeviceURLStore } from '../../../stores.js';

export function load({ params }) {
	let deviceId = params.slug;

	selectedDeviceIDStore.set(deviceId);
	selectedDeviceURLStore.set(`/devices/${deviceId}`)

	return {
		deviceId: deviceId
	};
}
