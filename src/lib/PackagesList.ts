import { listen } from '@tauri-apps/api/event';
import { get, type Unsubscriber } from 'svelte/store';
import type { DeviceUserPackage } from '../models';
import { notifications } from '../notificationStore';
import { packagesStore } from '../packageStore';
import { selectedUserStore } from '../stores';
import {
	adb_disable_package,
	adb_enable_package,
	adb_install_package,
	adb_list_packages
} from './adb';
import { setErrorModal } from './utils';

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
		packagesStore.setPackages(ep.device_id, ep.user_id, [ep.package]);
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
