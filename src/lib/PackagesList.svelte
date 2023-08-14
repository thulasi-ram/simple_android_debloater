<script>
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
		validateShouldRefreshPackageStore
	} from '../stores';

	import { onMount } from 'svelte';
	import { adb_list_packages } from './adb';

	onMount(() => {
		validateShouldRefreshPackageStore.subscribe((val) => {
			if (val) {
				adb_list_packages();
			}
		});
	});

	let searchTerm = '';
	$: filteredPackages = $packagesStore.filter(
		(pkg) => pkg.name.toLowerCase().indexOf(searchTerm.toLowerCase()) !== -1
	);
</script>

<div class="space-y-12">
	<SadError />

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
