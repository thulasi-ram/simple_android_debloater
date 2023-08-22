import { selectedDeviceStore } from '$lib/devices/stores';
import { derived, writable, type Readable, type Writable } from 'svelte/store';
import type { User } from './models';

export const selectedUserIDStore: Writable<string> = writable('');
export const selectedUserStore: Readable<User | null> = derived(
	[selectedDeviceStore, selectedUserIDStore],
	([$selectedDeviceStore, $selectedUserIDStore]) => {
		let selectedUserID = $selectedUserIDStore;

		if (selectedUserID === '') {
			return null;
		}

		let selectedUser: User | null = null;
		$selectedDeviceStore?.users.forEach((user) => {
			if (user.id === selectedUserID) {
				selectedUser = user;
				return;
			}
		});

		if (selectedUser == null) {
			selectedUserIDStore.set('');
		}
		return selectedUser;
	}
);