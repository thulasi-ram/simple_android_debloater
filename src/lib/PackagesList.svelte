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

	import { selectedUserStore } from '../stores';

	import { listen } from '@tauri-apps/api/event';
	import { onDestroy } from 'svelte';
	import { get } from 'svelte/store';
	import { notifications } from '../notificationStore';
	import { currentPackagesStore, packagesStore } from '../packageStore';
	import { adb_disable_package, adb_enable_package, adb_list_packages } from './adb';
	import { setErrorModal } from './utils';

	const unsubSelectedUserStore = selectedUserStore.subscribe((su) => {
		if (su) {
			if (!packagesStore.hasPackages(su.device_id, su.id)) {
				adb_list_packages(su.device_id, su.id)
					.then((pkgs) => {
						notifications.info(`fetching packages for ${su?.name}`);
						packagesStore.setPackages(pkgs.deviceId, pkgs.userId, pkgs.packages);
					})
					.catch(setErrorModal);
			}
		}
	});
	onDestroy(unsubSelectedUserStore);

	listen('package_event', (event) => {
		let ep = event.payload;
		packagesStore.setPackages(ep.device_id, ep.user_id, [ep.package]);
	});

	const disableSelectedPackage = (/** @type {string} */ pkg) => {
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
	};

	const enableSelectedPackage = (/** @type {string} */ pkg) => {
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
	};

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
