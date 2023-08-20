import { invoke } from '@tauri-apps/api/tauri';

import type { DeviceUserPackages, DeviceWithUsers, Package } from '../models';

export async function adb_list_devices_with_users(): Promise<DeviceWithUsers[]> {
	console.log(`invoking devices and users`);
	const cmdOutpt: [DeviceWithUsers] = await invoke('adb_list_devices_with_users');
	return cmdOutpt;
}

export async function adb_list_packages(
	deviceId: string,
	userId: string
): Promise<DeviceUserPackages> {
	console.log(`invoking packages - ${deviceId} - ${userId}`);

	const cmdOutpt: Package[] = await invoke('adb_list_packages', {
		deviceId: deviceId,
		userId: userId
	});
	return { device_id: deviceId, user_id: userId, packages: cmdOutpt };
}

export async function adb_disable_package(deviceId: string, userId: string, pkg: string) {
	console.log(`invoking disable - ${userId} - ${pkg}`);

	await invoke('adb_disable_clear_and_stop_package', {
		deviceId: deviceId,
		userId: userId,
		pkg: pkg
	});
}

export async function adb_enable_package(deviceId: string, userId: string, pkg: string) {
	console.log(`invoking enable - ${userId} - ${pkg}`);

	await invoke('adb_enable_package', {
		deviceId: deviceId,
		userId: userId,
		pkg: pkg
	});
}
