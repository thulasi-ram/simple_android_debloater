<script>
	import {
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
	import { adb_disable_packages, adb_list_packages } from './adb';

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

	const disableSelectedPackage = (/** @type {string} */ pkg) => {
		adb_disable_packages(pkg);
	};

	onDestroy(unsubSelectedUserStore);

	let searchTerm = '';
	$: filteredPackages = $currentPackagesStore.filter(
		(pkg) => pkg.name.toLowerCase().indexOf(searchTerm.toLowerCase()) !== -1
	);
</script>

<div class="space-y-12">
	<Table striped={true}>
		<TableSearch placeholder="Search by name" hoverable={true} bind:inputValue={searchTerm}>
			<TableHead>
				<TableHeadCell>name</TableHeadCell>
				<TableHeadCell>type</TableHeadCell>
				<TableHeadCell>state</TableHeadCell>
				<TableHeadCell>
					<span class="sr-only">actions</span>
				</TableHeadCell>
			</TableHead>
			<TableBody>
				{#each filteredPackages as pkg}
					<TableBodyRow>
						<TableBodyCell>{pkg.name}</TableBodyCell>
						<TableBodyCell>{pkg.ptype}</TableBodyCell>
						<TableBodyCell>{pkg.state}</TableBodyCell>
						<TableBodyCell>
							<TableBodyCell>
								<Button on:click={() => disableSelectedPackage(pkg.name)}>Disable</Button>
							</TableBodyCell></TableBodyCell
						>
					</TableBodyRow>
				{/each}
			</TableBody>
		</TableSearch>
	</Table>
</div>
