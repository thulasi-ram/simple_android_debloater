import { deviceHeartBeatCache } from '$lib/devices/cache';
import { derived, get, readable, writable, type Readable, type Writable } from 'svelte/store';
import type { DeviceWithUsers } from './models';

function createDeviceWithUsersStore() {
	const store = writable<Record<string, DeviceWithUsers>>({});
	const { set, update, subscribe } = store;

	function insertDevice(dev: DeviceWithUsers) {
		update((store) => {
			deviceHeartBeatCache.set(dev.device.id, new Date());
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

export const selectedDeviceIDStore: Writable<string> = writable('');

export const selectedDeviceStore: Readable<DeviceWithUsers | null> = derived(
	[devicesWithUsersStore, selectedDeviceIDStore],
	([$devicesWithUsersStore, $selectedDeviceIDStore]) => {
		let selectedDeviceID = $selectedDeviceIDStore;

		if (!selectedDeviceID) {
			return null;
		}

		let device = devicesWithUsersStore.getDevice(selectedDeviceID);

		return device;
	}
);

export const liveDevicesStore: Readable<Record<string, boolean>> = readable(
	{},
	function start(set) {
		const interval = setInterval(() => {
			let _store: Record<string, boolean> = {};

			for (let [deviceId, du] of Object.entries(get(devicesWithUsersStore))) {
				let isLive = false;

				if (du.device.state === 'Device') {
					if (deviceHeartBeatCache.get(deviceId) !== undefined) {
						isLive = true;
					}
				}
				_store[deviceId] = isLive;
			}

			set(_store);
		}, 1000);

		return function stop() {
			clearInterval(interval);
		};
	}
);
