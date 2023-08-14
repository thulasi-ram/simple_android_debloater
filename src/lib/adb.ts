import { invoke } from '@tauri-apps/api/tauri';

import {
	devicesWithUsersStore,
	sadErrorStore,
	selectedDeviceIDStore,
	selectedUserIDStore,
	packagesStore,
	packageGetDeviceAndUserIDStore,
	selectedDeviceStore,
	selectedUserStore
} from '../stores';
import type { DeviceWithUsers, Package } from '../models';
import { get } from 'svelte/store';
import { notifications } from '../notificationStore';

export async function adb_list_devices_with_users() {
	sadErrorStore.reset();
	notifications.info('fetching devices and users');
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

	notifications.info('fetching packages');

	packageGetDeviceAndUserIDStore.set({
		deviceId: selectedDevice.device.id,
		userId: selectedUser.id
	});

	try {
		const cmdOutpt: Package[] = await invoke('adb_list_packages', {
			deviceId: selectedDevice.device.id,
			userId: selectedUser.id
		});
		packagesStore.set(cmdOutpt);
	} catch (e) {
		sadErrorStore.setError(String(e), true);
	}
}
