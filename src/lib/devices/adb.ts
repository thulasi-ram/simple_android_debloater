import { invoke } from '@tauri-apps/api/tauri';
import { info } from "tauri-plugin-log-api";
import type { DeviceWithUsers } from './models';

export async function adb_list_devices_with_users(): Promise<DeviceWithUsers[]> {
	info(`invoking devices and users`);
	const cmdOutpt: [DeviceWithUsers] = await invoke('adb_list_devices_with_users');
	return cmdOutpt;
}
