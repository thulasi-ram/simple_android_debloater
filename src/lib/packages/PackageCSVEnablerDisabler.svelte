<script lang="ts">
	import { devicesWithUsersStore } from '$lib/devices/stores';
	import { sadErrorStore } from '$lib/error/stores';
	import { notifications } from '$lib/notifications/stores';
	import type { Package } from '$lib/packages/models';
	import { sleep } from '$lib/utils';
	import {
		Button,
		Checkbox,
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
	import { savePackagesResultsCSV, type PackageNames } from '../../routes/import/import';
	import { IconDownload } from '@tabler/icons-svelte';

	export let res: () => Promise<PackageNames> | undefined;
	export let title: string;
	export let open: boolean;
	export let processor: (deviceID: string, userID: string, pkg: string) => Promise<Package>;

	let packagesStatusMap: Record<string, string> = {};

	async function packagesPromise() {
		let packages = await res();
		if (packages) {
			packages.forEach((p) => {
				packagesStatusMap[p] = 'unprocessed';
			});
		}
	}

	let selectedDeviceID: string | undefined;
	let selectedUserID: string | undefined;
	$: devices = Object.entries($devicesWithUsersStore).map(([_, du]) => {
		return { name: du.device.name, value: du.device.id };
	});
	let users: { name: string; value: string }[] = [];

	$: {
		if (selectedDeviceID && !selectedUserID) {
			let du = devicesWithUsersStore.getDevice(selectedDeviceID);
			if (du) {
				users = du?.users.map((u) => {
					return { name: u.name, value: u.id };
				});
			}
		}
	}

	let processingCount = 0;
	let stopProcessing = false;
	let iAgreeProcessing = false;

	$: {
		if (processingCount <= 0 && stopProcessing) {
			// reset stopprocessing if no items are being processed
			stopProcessing = false;
		}
	}

	function onContinueButton() {
		Object.entries(packagesStatusMap).forEach(([k, v]) => {
			processingCount += 1;
			packagesStatusMap[k] = 'processing';

			sleep(500).then(() => {
				if (stopProcessing) {
					packagesStatusMap[k] = 'unprocessed|stopped';
					processingCount -= 1;
					return;
				}

				sleep(500).then(() => {
					console.log('procceding');
					if (!selectedDeviceID) {
						packagesStatusMap[k] = 'unprocessed|device_id_empty';
						processingCount -= 1;
						return;
					}

					if (!selectedUserID) {
						packagesStatusMap[k] = 'unprocessed|user_id_empty';
						processingCount -= 1;
						return;
					}

					processor(selectedDeviceID, selectedUserID, k)
						.then((p) => {
							packagesStatusMap[k] = `processed|${p.state}`;
							processingCount -= 1;
							console.log('processed', p);
						})
						.catch((e) => {
							packagesStatusMap[k] = `error|${e}`;
							processingCount -= 1;
							console.error(e);
						});
				});
			});
		});
	}

	function downloadPackageResults() {
		savePackagesResultsCSV(
			Object.entries(packagesStatusMap).map(([k, v]) => {
				return { package: k, result: v };
			})
		)
			.then(() => {
				notifications.success('Package results exported');
			})
			.catch((e) => {
				sadErrorStore.setError(e);
			});
	}
</script>

<Modal {title} bind:open>
	{#await packagesPromise()}
		Parsing CSV File...
	{:then}
		<div class="flex flex-row gap-x-5">
			<Label>
				Select a device
				<Select class="mt-2" items={devices} bind:value={selectedDeviceID} />
			</Label>

			<Label>
				Select a user
				<Select class="mt-2" items={users} bind:value={selectedUserID} />
			</Label>
		</div>

		<Table shadow hoverable={true}>
			<TableHead>
				<TableHeadCell>Packages</TableHeadCell>
				<TableHeadCell>Status</TableHeadCell>
			</TableHead>
			<TableBody>
				{#each Object.entries(packagesStatusMap) as [p, status]}
					<TableBodyRow class="border rounded">
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
		<div class="flex flex-col gap-y-5">
			<Checkbox bind:checked={iAgreeProcessing}>
				<span class="text-sm text-red-700">
					This is a potentially dangerous action and can brick the device. I agree to process.
				</span>
			</Checkbox>

			<div class="flex gap-x-5">
				{#if processingCount > 0}
					<Button
						outline
						color="red"
						on:click={() => {
							stopProcessing = true;
						}}
					>
						{#if stopProcessing && processingCount > 0}
							Stopping...
						{:else}
							Stop Processing
						{/if}
					</Button>
				{:else}
					<Button disabled={!iAgreeProcessing} on:click={onContinueButton}>Process {title}</Button>
				{/if}

				<div class="mx-auto" />

				<Button class="gap-x-2" outline color="light" on:click={downloadPackageResults}>
					<IconDownload stroke={1.5} size={21} />
					Download Results
				</Button>
			</div>
		</div>
	</svelte:fragment>
</Modal>
