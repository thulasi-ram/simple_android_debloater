import { invoke } from '@tauri-apps/api/tauri';

import { devicesWithUsersStore, sadErrorStore } from '../stores';
import type { DeviceWithUsers } from '../models';

export async function adb_list_devices_with_users() {
	sadErrorStore.reset();
	try {
		const cmdOutpt: [DeviceWithUsers] = await invoke('adb_list_devices_with_users');
		devicesWithUsersStore.set(cmdOutpt);
	} catch (e) {
		sadErrorStore.setError(String(e));
	}
}
