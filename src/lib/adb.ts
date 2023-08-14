import { invoke } from '@tauri-apps/api/tauri';

import {
	devicesWithUsersStore,
	sadErrorStore,
	selectedDeviceIDStore,
	selectedUserIDStore,
	packagesStore
} from '../stores';
import type { DeviceWithUsers, Package } from '../models';
import { get } from 'svelte/store';

export async function adb_list_devices_with_users() {
	sadErrorStore.reset();
	try {
		const cmdOutpt: [DeviceWithUsers] = await invoke('adb_list_devices_with_users');
		devicesWithUsersStore.set(cmdOutpt);
	} catch (e) {
		sadErrorStore.setError(String(e));
	}
}

export async function adb_list_packages() {
	if (!selectedDeviceIDStore) {
		return sadErrorStore.setError('Device ID is not yet set', false);
	}

	if (!selectedUserIDStore.toString()) {
		return sadErrorStore.setError('User is not yet set', false);
	}

	try {
		const cmdOutpt: [Package] = await invoke('adb_list_packages', {
			deviceId: get(selectedDeviceIDStore),
			userId: get(selectedUserIDStore).toString()
		});
		packagesStore.set(cmdOutpt);
	} catch (e) {
		sadErrorStore.setError(String(e), true);
	}
}
