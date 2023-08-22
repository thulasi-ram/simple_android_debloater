import { invoke } from '@tauri-apps/api/tauri';

import type { DeviceWithUsers } from './models';

export async function adb_list_devices_with_users(): Promise<DeviceWithUsers[]> {
	console.log(`invoking devices and users`);
	const cmdOutpt: [DeviceWithUsers] = await invoke('adb_list_devices_with_users');
	return cmdOutpt;
}
