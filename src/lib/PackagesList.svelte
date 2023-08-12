<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { Button } from 'flowbite-svelte';
	import SadError from './SadError.svelte';

	import {
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow,
		TableHead,
		TableHeadCell,
		TableSearch
	} from 'flowbite-svelte';

	import {
		packagesStore,
		sadErrorStore,
		selectedDeviceIDStore,
		selectedUserIDStore
	} from '../stores';

	import { onMount } from 'svelte';

	async function adb_list_packages() {
		if (!$selectedDeviceIDStore) {
			return sadErrorStore.setError('Device ID is not yet set', false);
		}

		if (!$selectedUserIDStore.toString()) {
			return sadErrorStore.setError('User is not yet set', false);
		}

		try {
			const cmdOutpt = await invoke('adb_list_packages', {
				deviceId: $selectedDeviceIDStore,
				userId: $selectedUserIDStore.toString()
			});
			packagesStore.set(cmdOutpt);
		} catch (e) {
			sadErrorStore.setError(String(e), true);
		}
	}

	onMount(() => {
		setTimeout(() => {
			adb_list_packages();
		}, 500);
	});

	let searchTerm = '';
	$: filteredPackages = $packagesStore.filter(
		(pkg) => pkg.name.toLowerCase().indexOf(searchTerm.toLowerCase()) !== -1
	);
</script>

<div class="space-y-12">
	<SadError />

	<Button on:click={adb_list_packages}>ADB List Packages</Button>

	<Table striped={true}>
		<TableSearch placeholder="Search by name" hoverable={true} bind:inputValue={searchTerm}>
			<TableHead>
				<TableHeadCell>name</TableHeadCell>
				<TableHeadCell>type</TableHeadCell>
				<TableHeadCell>state</TableHeadCell>
			</TableHead>
			<TableBody>
				{#each filteredPackages as pkg}
					<TableBodyRow>
						<TableBodyCell>{pkg.name}</TableBodyCell>
						<TableBodyCell>{pkg.ptype}</TableBodyCell>
						<TableBodyCell>{pkg.state}</TableBodyCell>
					</TableBodyRow>
				{/each}
			</TableBody>
		</TableSearch>
	</Table>
</div>
