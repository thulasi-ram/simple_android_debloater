import { selectedDeviceIDStore } from "$lib/devices/stores";

export function load({ params }) {
	let deviceId = params.slug;

	selectedDeviceIDStore.set(deviceId);
	return {
		deviceId: deviceId
	};
}
