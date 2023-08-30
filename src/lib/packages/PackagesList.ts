import { setErrorModal } from '$lib/error';
import { selectedUserStore } from '$lib/users/stores';
import { listen } from '@tauri-apps/api/event';
import { get, type Unsubscriber } from 'svelte/store';
import { notifications } from '../notifications/stores';
import {
	adb_disable_package,
	adb_enable_package,
	adb_install_package,
	adb_list_packages
} from './adb';
import type { DeviceUserPackage, Package } from './models';
import { packagesStore } from './stores';

export const discussion_create_url = (pkg: Package) => {
	// https://eric.blog/2016/01/08/prefilling-github-issues/#:~:text=Creating%20issues&text=As%20long%20as%20you're,using%20and%20testing%20your%20software.
	let body = encodeURI('<describe the package>');
	return `https://github.com/thulasi-ram/simple_android_debloater/discussions/new?category=packages&title=${pkg.name}&body=${body}`;
};

export function fetchPackagesIfEmptySubscription(): Unsubscriber {
	const unsub = selectedUserStore.subscribe((su) => {
		if (su) {
			if (!packagesStore.hasPackages(su.device_id, su.id)) {
				adb_list_packages(su.device_id, su.id)
					.then((pkgs) => {
						notifications.info(`fetched packages for ${su?.name}`);
						packagesStore.setPackages(pkgs.device_id, pkgs.user_id, pkgs.packages);
					})
					.catch(setErrorModal);
			}
		}
	});
	return unsub;
}

export async function packageEventListener() {
	await listen('package_event', (event) => {
		let ep = event.payload as DeviceUserPackage;
		packagesStore.addPackage(ep.device_id, ep.user_id, ep.package);
	});
}

export function disablePackage(pkg: string) {
	let user = get(selectedUserStore);
	if (!user) {
		return setErrorModal('user is not selected');
	}

	notifications.info(`disabling package: {pkg} - ${user.name} ${pkg}`);

	adb_disable_package(user.device_id, user.id, pkg)
		.then(() => {
			notifications.success(`${pkg} successfully disabled`);
		})
		.catch((e) => {
			notifications.error(`error disabling ${pkg} - ${JSON.stringify(e)}`);
		});
}

export function enablePackage(pkg: string) {
	let user = get(selectedUserStore);
	if (!user) {
		return setErrorModal('user is not selected');
	}
	notifications.info(`enabling package: {pkg} - ${user.name} ${pkg}`);

	adb_enable_package(user.device_id, user.id, pkg)
		.then(() => {
			notifications.success(`${pkg} successfully enabled`);
		})
		.catch((e) => {
			notifications.error(`error enabling ${pkg} - ${JSON.stringify(e)}`);
		});
}

export function installPackage(pkg: string) {
	let user = get(selectedUserStore);
	if (!user) {
		return setErrorModal('user is not selected');
	}
	notifications.info(`installing package: {pkg} - ${user.name} ${pkg}`);

	adb_install_package(user.device_id, user.id, pkg)
		.then(() => {
			notifications.success(`${pkg} successfully installed`);
		})
		.catch((e) => {
			notifications.error(`error installing ${pkg} - ${JSON.stringify(e)}`);
		});
}
