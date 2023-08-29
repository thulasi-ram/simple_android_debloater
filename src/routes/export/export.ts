import type { Device, DeviceWithUsers } from '$lib/devices/models';
import { devicesWithUsersStore, selectedDeviceStore } from '$lib/devices/stores';
import { setErrorModal } from '$lib/error';
import type { Package } from '$lib/packages/models';
import { currentPackagesStore, packagesKey, packagesStore } from '$lib/packages/stores';
import type { User } from '$lib/users/models';
import { selectedUserStore } from '$lib/users/stores';
import { save } from '@tauri-apps/api/dialog';
import { writeTextFile } from '@tauri-apps/api/fs';
import Papa from 'papaparse';
import { get } from 'svelte/store';

type DeviceUserExport = {
	device_id: string | undefined;
	device_name: string | undefined;
	user_id: string | undefined;
	user_name: string | undefined;
};

type PackageExportCSV = Package & DeviceUserExport;

type UserWithPackages = {
	user: User;
	packages: Package[];
};

type DeviceWithUserPackages = {
	device: Device;
	users: UserWithPackages[];
};

type PackageExportJSON = DeviceWithUserPackages[];

async function saveCSV(data: PackageExportCSV[]) {
	const savePath = await save({
		title: 'Save Packages Export CSV',
		filters: [{ name: 'Comma Seperated Values', extensions: ['csv'] }]
	});
	if (!savePath) return;

	const fcontent = Papa.unparse(data);

	await writeTextFile(savePath, fcontent);
}

async function saveJSON(data: PackageExportJSON) {
	const savePath = await save({
		title: 'Save Packages Export JSON',
		filters: [{ name: 'JSON', extensions: ['json'] }]
	});
	if (!savePath) return;

	const fcontent = JSON.stringify(data);

	await writeTextFile(savePath, fcontent);
}

export async function exportCSV() {
	let device = get(selectedDeviceStore);
	if (!device) {
		throw new Error('device must be selected for export');
	}

	let user = get(selectedUserStore);
	if (!user) {
		throw new Error('user must be selected for export');
	}

	let packages = get(currentPackagesStore);

	let exportablePackages: PackageExportCSV[] = packages.map((p) => {
		return {
			name: p.name,
			ptype: p.ptype,
			state: p.state,
			package_prefix: p.package_prefix,
			device_id: device?.device.id,
			device_name: device?.device.name,
			user_id: user?.id,
			user_name: user?.name
		};
	});

	await saveCSV(exportablePackages);
}

export async function exportJSON() {
	let deviceWithUsers = get(devicesWithUsersStore);

	let allPackages = get(packagesStore);

	let exportabledeviceWithUsers: DeviceWithUserPackages[] = Object.entries(deviceWithUsers).map(
		([_, du]) => {
			return {
				device: du.device,
				users: du.users.map((u) => {
					let pkey = packagesKey(du.device.id, u.id);
					let packages = pkey ? allPackages[pkey] : [];
					return {
						user: u,
						packages: packages || []
					};
				})
			};
		}
	);

	await saveJSON(exportabledeviceWithUsers);
}
