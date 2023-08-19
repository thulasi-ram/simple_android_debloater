import { error } from '@sveltejs/kit';
import { derived, get, writable } from 'svelte/store';
import type { Package } from './models';
import { selectedDeviceStore, selectedUserStore } from './stores';
import { setErrorModal } from '$lib/utils';

function createPackagesStore() {
	const store = writable<Record<string, Package[]>>({});
	const { set, update, subscribe } = store;

	function setPackages(device_id: string, user_id: string, packages: Package[]) {


		update((store) => {
			let pkey = packagesKey(device_id, user_id);
			if (!pkey) {
				setErrorModal("pkey is empty")
				return store;
			}
			store[pkey] = packages;
			return store;
		});
	}

	function hasPackages(device_id: string, user_id: string): boolean {
		let pkey = packagesKey(device_id, user_id);
		if (!pkey) {
			return false;
		}
		return get(store).hasOwnProperty(pkey);
	}

	return {
		subscribe,
		setPackages,
		hasPackages
	};
}

export const packagesStore = createPackagesStore();

const packagesKey = (deviceId: string | undefined, userId: string | undefined): string | null => {
	if (!deviceId || !userId) {
		console.log("pkey is null", deviceId, userId)
		return null;
	}
	return `${deviceId}-${userId}`;
};

export const currentPackagesStore = derived(
	[packagesStore, selectedDeviceStore, selectedUserStore],
	([$packagesStore, $selectedDeviceStore, $selectedUserStore]) => {
		let user = $selectedUserStore;
		let pkey = packagesKey(user?.device_id, user?.id);
		if (!pkey) {
			return [];
		}
		let pkgs = $packagesStore[pkey];
		return pkgs || [];
	}
);
