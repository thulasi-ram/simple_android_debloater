import { type Writable, writable, derived } from 'svelte/store';
import type { DeviceWithUsers, Package, User } from './models';

export const devicesWithUsersStore: Writable<[DeviceWithUsers]> = writable(
	<[DeviceWithUsers]>(<unknown>[])
);

export const packagesStore: Writable<[Package]> = writable(
	<[Package]>(<unknown>[])
);

export const selectedDeviceIDStore: Writable<string> = writable('');
export const selectedUserIDStore: Writable<string> = writable('');

export const applicableUsersStore = derived(
	[devicesWithUsersStore, selectedDeviceIDStore],
	([$devicesWithUsers, $selectedDeviceIDStore]) => {

		if ($selectedDeviceIDStore == '') {
			return <unknown>[] as [User];
		}

		for (let d of $devicesWithUsers) {
			console.log('D', $selectedDeviceIDStore, d.device.id);
			if (d.device.id == $selectedDeviceIDStore) {
				return d.users;
			}
		}

		return <unknown>[] as [User];
	}
);
