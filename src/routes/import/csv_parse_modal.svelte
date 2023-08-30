<script lang="ts">
	import {
		Button,
		Label,
		Modal,
		Select,
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow,
		TableHead,
		TableHeadCell
	} from 'flowbite-svelte';
	import type { PackageNames } from './import';
	import { devicesWithUsersStore } from '$lib/devices/stores';
	import { adb_disable_package } from '$lib/packages/adb';

	export let res: () => Promise<PackageNames> | undefined;
	export let title: string;

	export let open: boolean;

	let packages: PackageNames | undefined;
	let packagesStatusMap: Record<string, string> = {};

	async function packagesPromise() {
		packages = await res();
		if (packages) {
			packages.forEach((p) => {
				packagesStatusMap[p] = 'unprocessed';
			});
		}
	}

	let selectedDeviceID: string | undefined;
	let selectedUserID: string | undefined;
	let devices = Object.entries($devicesWithUsersStore).map(([_, du]) => {
		return { name: du.device.name, value: du.device.id };
	});
	let users: { name: string; value: string }[] = [];

	$: {
		if (selectedDeviceID) {
			let du = devicesWithUsersStore.getDevice(selectedDeviceID);
			if (du) {
				users = du?.users.map((u) => {
					return { name: u.name, value: u.id };
				});
			}
		}
	}

	function onContinueButton() {
		Object.entries(packagesStatusMap).forEach(([k, v]) => {
			if (!selectedDeviceID) {
				return;
			}

			if (!selectedUserID) {
				return;
			}
			adb_disable_package(selectedDeviceID, selectedUserID, k)
				.then((p) => {
					console.log('processed');
					packagesStatusMap[k] = `processed|${p.state}`;
				})
				.catch((e) => {
					console.error(e);
				});
		});
	}
</script>

<Modal {title} bind:open>
	{#await packagesPromise()}
		ParsingFile
	{:then}
		<Label>
			Select a device
			<Select class="mt-2" items={devices} bind:value={selectedDeviceID} />
		</Label>

		<Label>
			Select a user
			<Select class="mt-2" items={users} bind:value={selectedUserID} />
		</Label>

		<Table shadow hoverable={true}>
			<TableHead>
				<TableHeadCell>Packages</TableHeadCell>
				<TableHeadCell>Status</TableHeadCell>
			</TableHead>
			<TableBody>
				{#each Object.entries(packagesStatusMap) as [p, status]}
					<TableBodyRow>
						<TableBodyCell>{p}</TableBodyCell>
						<TableBodyCell>{status}</TableBodyCell>
					</TableBodyRow>
				{/each}
			</TableBody>
		</Table>
	{:catch error}
		{error}
	{/await}

	<svelte:fragment slot="footer">
		<div>
			<button on:click|preventDefault={onContinueButton}>Continue</button>
		</div>
	</svelte:fragment>
</Modal>
