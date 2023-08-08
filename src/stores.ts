import { type Writable, writable, derived } from 'svelte/store';
import type { DeviceWithUsers, User } from './models';

export const devicesWithUsersStore: Writable<[DeviceWithUsers]> = writable(
	<[DeviceWithUsers]>(<unknown>[])
);

export const selectedDeviceIDStore: Writable<string> = writable('');
export const selectedUserIDStore: Writable<string> = writable('');

export const applicableUsersStore = derived(
	[devicesWithUsersStore, selectedDeviceIDStore],
	([$devicesWithUsers, $selectedDeviceIDStore]) => {

		if ($selectedDeviceIDStore == '') {
			return {} as [User];
		}

		for (let d of $devicesWithUsers) {
			console.log('D', $selectedDeviceIDStore, d.device.id);
			if (d.device.id == $selectedDeviceIDStore) {
				return d.users;
			}
		}

		return {} as [User];
	}
);
