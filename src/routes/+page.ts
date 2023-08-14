import { adb_list_devices_with_users } from '$lib/adb';

export async function load({}) {
	await adb_list_devices_with_users();
}
