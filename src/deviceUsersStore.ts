import { get, writable } from 'svelte/store';
import type { DeviceWithUsers } from './models';

function createDeviceWithUsersStore() {
	const store = writable<Record<string, DeviceWithUsers>>({});
	const { set, update, subscribe } = store;

	function insertDevice(dev: DeviceWithUsers) {
		update((store) => {
			store[dev.device.id] = dev;
			return store;
		});
	}

	function hasDevice(device_id: string) {
		return get(store).hasOwnProperty(device_id);
	}

	function getDevice(device_id: string): DeviceWithUsers | null {
		let devices = get(store);
		for (const key in devices) {
			if (key === device_id) {
				return devices[key];
			}
		}
		return null;
	}

	return {
		subscribe,
		insertDevice,
		getDevice
	};
}

export const devicesWithUsersStore = createDeviceWithUsersStore();
