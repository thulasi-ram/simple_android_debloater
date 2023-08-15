import { error } from '@sveltejs/kit';
import { derived, writable } from 'svelte/store';
import type { Package } from './models';
import { selectedDeviceStore, selectedUserStore } from './stores';

function createPackagesStore() {
	const { set, update, subscribe } = writable<Record<string, Package[]>>({});

	function setPackages(pkey: string, packages: Package[]) {
		update((store) => {
			store[pkey] = packages;
			return store;
		});
	}

	return {
		subscribe,
		setPackages
	};
}

export const packagesStore = createPackagesStore();

export const packagesKey = (deviceID: string | undefined, userId: string | undefined) => {
	if (!deviceID || !userId) {
		throw error(400, 'deviceid or userid cannot be empty');
	}
	return `${deviceID}-${userId}`;
};

export const currentPackagesStore = derived(
	[packagesStore, selectedDeviceStore, selectedUserStore],
	([$packagesStore, $selectedDeviceStore, $selectedUserStore]) => {
		let pkey = packagesKey($selectedDeviceStore?.device.id, $selectedUserStore?.id);
		let pkgs = $packagesStore[pkey];
		return pkgs || [];
	}
);
