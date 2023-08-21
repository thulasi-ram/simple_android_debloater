import { derived, writable, type Readable, type Writable } from 'svelte/store';
import { devicesWithUsersStore } from './deviceUsersStore';
import type { DeviceWithUsers, User } from './models';

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
			selectedUserIDStore.set("");
		}
		return selectedUser;
	}
);

export const selectedDeviceURLStore: Writable<string> = writable('');

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
