<script>
	import {
		Badge,
		Button,
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow,
		TableHead,
		TableHeadCell,
		TableSearch
	} from 'flowbite-svelte';

	import { selectedDeviceStore, selectedUserStore } from '../stores';

	import { onDestroy } from 'svelte';
	import { currentPackagesStore, packagesKey, packagesStore } from '../packageStore';
	import { adb_disable_package, adb_enable_package, adb_list_packages } from './adb';
	import { listen } from '@tauri-apps/api/event';
	import { notifications } from '../notificationStore';

	const unsubSelectedUserStore = selectedUserStore.subscribe((su) => {
		if (su) {
			let hasPackages = $packagesStore.hasOwnProperty(
				packagesKey($selectedDeviceStore?.device.id, su.id)
			);
			if (!hasPackages) {
				adb_list_packages();
			}
		}
	});

	listen('package_event', (event) => {
		let ep = event.payload;
		let packages = [ep.package];
		packagesStore.setPackages(packagesKey(ep.device_id, ep.user_id), packages);
	});

	const disableSelectedPackage = (/** @type {string} */ pkg) => {
		adb_disable_package(pkg)
			.then(() => {
				notifications.success(`${pkg} successfully disabled`);
			})
			.catch((e) => {
				notifications.error(`error disabling ${pkg} - ${JSON.stringify(e)}`);
			});
	};

	const enableSelectedPackage = (/** @type {string} */ pkg) => {
		adb_enable_package(pkg)
			.then(() => {
				notifications.success(`${pkg} successfully enabled`);
			})
			.catch((e) => {
				notifications.error(`error enabling ${pkg} - ${JSON.stringify(e)}`);
			});
	};

	onDestroy(unsubSelectedUserStore);

	let searchTerm = 'aaaaxy';
	$: filteredPackages = $currentPackagesStore.filter(
		(pkg) => pkg.name.toLowerCase().indexOf(searchTerm.toLowerCase()) !== -1
	);

	let tbCellClass = 'whitespace-nowrap font-small px-6 py-1';
</script>

<div class="space-y-12">
	<Table striped={true}>
		<TableSearch placeholder="Search by name" hoverable={true} bind:inputValue={searchTerm}>
			<TableHead>
				<TableHeadCell>name</TableHeadCell>
				<TableHeadCell>
					<span class="sr-only">actions</span>
				</TableHeadCell>
			</TableHead>
			<TableBody>
				{#each filteredPackages as pkg}
					<TableBodyRow>
						<TableBodyCell tdClass={tbCellClass}>
							{pkg.name}
							{#if pkg.ptype == 'system'}
								<Badge rounded color="green">{pkg.ptype}</Badge>
							{:else}
								<Badge rounded color="primary">{pkg.ptype}</Badge>
							{/if}
						</TableBodyCell>
						<TableBodyCell tdClass={tbCellClass}>
							{#if pkg.state == 'Enabled'}
								<Button
									size="xs"
									outline
									color="red"
									class="rounded"
									on:click={() => disableSelectedPackage(pkg.name)}>Disable</Button
								>
							{:else}
								<Button
									size="xs"
									outline
									color="green"
									class="rounded"
									on:click={() => enableSelectedPackage(pkg.name)}>Enable</Button
								>
							{/if}
						</TableBodyCell>
					</TableBodyRow>
				{/each}
			</TableBody>
		</TableSearch>
	</Table>
</div>
