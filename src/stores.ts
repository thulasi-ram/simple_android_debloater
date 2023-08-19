import { derived, writable, type Readable, type Writable } from 'svelte/store';
import type { DeviceWithUsers, Package, User } from './models';
import { devicesWithUsersStore } from './deviceUsersStore';

export const selectedDeviceIDStore: Writable<string> = writable('');

export const selectedDeviceStore: Readable<DeviceWithUsers | null> = derived(
	[devicesWithUsersStore, selectedDeviceIDStore],
	([$devicesWithUsersStore, $selectedDeviceIDStore]) => {
		let selectedDeviceID = $selectedDeviceIDStore;

		if (!selectedDeviceID) {
			return null;
		}

		let device = devicesWithUsersStore.getDevice(selectedDeviceID);

		return device;
	}
);

export const selectedUserIDStore: Writable<string> = writable('');
export const selectedUserStore: Readable<User | null> = derived(
	[selectedDeviceStore, selectedUserIDStore],
	([$selectedDeviceStore, $selectedUserIDStore]) => {
		let selectedUserID = $selectedUserIDStore;

		if (selectedUserID === '') {
			return;
		}

		let selectedUser;
		$selectedDeviceStore?.users.forEach((user) => {
			if (user.id === selectedUserID) {
				selectedUser = user;
				return;
			}
		});

		return selectedUser;
	}
);

export const selectedSidebarItemStore: Writable<string> = writable('');

type SadError = {
	message: string;
	isPermanent: boolean;
};

function createSadErrorStore() {
	const { set, update, subscribe } = writable<SadError>({
		message: '',
		isPermanent: false
	});

	function setError(message: string, isPermanent: boolean = false) {
		update((store) => ({
			...store,
			message: message,
			isPermanent: isPermanent
		}));
	}

	return {
		subscribe,
		setError,
		reset: () => setError('', false)
	};
}

export const sadErrorStore = createSadErrorStore();
