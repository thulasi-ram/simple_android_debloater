import { invoke } from '@tauri-apps/api/tauri';

import { get } from 'svelte/store';
import type { DeviceWithUsers, Package } from '../models';
import { notifications } from '../notificationStore';
import { packagesKey, packagesStore } from '../packageStore';
import {
	devicesWithUsersStore,
	sadErrorStore,
	selectedDeviceStore,
	selectedUserStore
} from '../stores';

export async function adb_list_devices_with_users() {
	sadErrorStore.reset();
	notifications.info('fetching devices and users');
	console.log(`invoking devices and users`);
	try {
		const cmdOutpt: [DeviceWithUsers] = await invoke('adb_list_devices_with_users');
		devicesWithUsersStore.set(cmdOutpt);
	} catch (e) {
		sadErrorStore.setError(String(e));
	}
}

export async function adb_list_packages() {
	let selectedDevice = get(selectedDeviceStore);
	let selectedUser = get(selectedUserStore);

	if (!selectedDevice) {
		return sadErrorStore.setError('Device is not selected', false);
	}

	if (!selectedUser) {
		return sadErrorStore.setError('User is not yet set', false);
	}

	notifications.info(`fetching packages for ${selectedDevice.device.name} - ${selectedUser.name}`);

	console.log(`invoking packages - ${selectedDevice.device.id} - ${selectedUser.name}`);

	try {
		const cmdOutpt: Package[] = await invoke('adb_list_packages', {
			deviceId: selectedDevice.device.id,
			userId: selectedUser.id
		});
		packagesStore.setPackages(packagesKey(selectedDevice.device.id, selectedUser.id), cmdOutpt);
	} catch (e) {
		sadErrorStore.setError(String(e), true);
	}
}
