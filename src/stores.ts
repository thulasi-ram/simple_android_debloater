import { derived, writable, type Readable, type Writable } from 'svelte/store';
import type { DeviceWithUsers, Package, User } from './models';

export const devicesWithUsersStore: Writable<DeviceWithUsers[]> = writable([]);

export const packagesStore: Writable<Package[]> = writable([]);

export const selectedDeviceIDStore: Writable<string> = writable('');
export const selectedDeviceStore: Readable<DeviceWithUsers | null> = derived(
	[devicesWithUsersStore, selectedDeviceIDStore],
	([$devicesWithUsers, $selectedDeviceIDStore]) => {
		let selectedDeviceID = $selectedDeviceIDStore;

		if (!selectedDeviceID) {
			return;
		}

		let selectedDevice;
		$devicesWithUsers.forEach((dev) => {
			if (dev.device.id === selectedDeviceID) {
				selectedDevice = dev;
				return;
			}
		});

		return selectedDevice;
	}
);

export const selectedUserIDStore: Writable<string> = writable('');
export const selectedUserStore: Readable<User | null> = derived(
	[selectedDeviceStore, selectedUserIDStore],
	([$selectedDeviceStore, $selectedUserIDStore]) => {
		let selectedUserID = $selectedUserIDStore;

		if (selectedUserID === "") {
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

export const packageGetDeviceAndUserIDStore = writable({
	deviceId: '',
	userId: ''
});

export const validateShouldRefreshPackageStore = derived(
	[packageGetDeviceAndUserIDStore, selectedDeviceStore, selectedUserStore],

	([$packageGetDeviceAndUserID, $selectedDeviceStore, $selectedUserStore]) => {
		let [sDeviceId, sUserId, cDeviceID, cUserID] = [
			$selectedDeviceStore?.device.id,
			$selectedUserStore?.id,
			$packageGetDeviceAndUserID.deviceId,
			$packageGetDeviceAndUserID.userId
		];

		// console.log(sDeviceId, sUserId, cDeviceID, cUserID);

		if (sDeviceId && sUserId) {
			if (!cDeviceID && cUserID === "") {
				// return true since we did not get earlier
				return true;
			}

			if (cDeviceID !== sDeviceId || cUserID !== sUserId) {
				return true;
			}
		}

		return false;
	}
);

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
