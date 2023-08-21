<script lang="ts">
	import { Badge, Button, Checkbox, Input, Modal } from 'flowbite-svelte';

	import type { Package } from '../models';
	import { currentPackagesStore, filteredPackages } from '../packageStore';

	import { IconFilter, IconSearch } from '@tabler/icons-svelte';

	let filterModalOpen = false;

	let searchTerm = '';

	let filterPackageStates: [string, string][] = [
		['Disabled', 'Disabled'],
		['Enabled', 'Enabled'],
		['Uninstalled', 'Uninstalled'],
		['Hidden', 'Hidden']
	];
	let selectedFilterPackageStates: string[] = [];

	let filterPackageTypes: [string, string][] = [
		['System', 'system'],
		['Third Party', 'thirdparty'],
		['Unknown', 'unknown']
	];
	let selectedFilterPackageTypes: string[] = [];

	let selectedFiltersCount = 0;

	$: {
		selectedFiltersCount = 0;

		if (selectedFilterPackageStates) {
			selectedFiltersCount += selectedFilterPackageStates.length;
		}

		if (selectedFilterPackageTypes) {
			selectedFiltersCount += selectedFilterPackageTypes.length;
		}
	}

	$: {
		let fpkgs: Package[] = $currentPackagesStore;

		if (searchTerm) {
			fpkgs = fpkgs.filter(
				(pkg) => pkg.name.toLowerCase().indexOf(searchTerm.toLowerCase()) !== -1
			);
		}

		if (selectedFilterPackageStates.length > 0) {
			let fstatesFiltered: Package[] = [];
			for (let fs of selectedFilterPackageStates) {
				let fspkgs = fpkgs.filter(
					(pkg) => pkg.state.toLowerCase().indexOf(fs.toLowerCase()) !== -1
				);
				fstatesFiltered.push(...fspkgs);
			}
			fpkgs = fstatesFiltered;
		}

		if (selectedFilterPackageTypes.length > 0) {
			let ftypesFiltered: Package[] = [];
			for (let ft of selectedFilterPackageTypes) {
				let ftpkgs = fpkgs.filter(
					(pkg) => pkg.ptype.toLowerCase().indexOf(ft.toLowerCase()) !== -1
				);
				ftypesFiltered.push(...ftpkgs);
			}
			fpkgs = ftypesFiltered;
		}

		$filteredPackages = fpkgs;
	}
</script>

<div class="flex items-center gap-x-2">
	<div>
		<div class="relative">
			<Input type="search" class="pl-10" placeholder="Search..." bind:value={searchTerm}>
				<svelte:fragment slot="left">
					<IconSearch size={18} stroke={1.5} />
				</svelte:fragment>
			</Input>
		</div>
	</div>

	<div>
		<Button color="alternative" on:click={() => (filterModalOpen = true)}>
			<IconFilter size={18} stroke={1.5} />
			<span class="mx-1">Filters</span>
			<Badge border class="px-2 mx-1">{selectedFiltersCount}</Badge>
		</Button>

		<!-- https://learn.svelte.dev/tutorial/multiple-select-bindings -->
		<!-- https://flowbite-svelte-blocks.vercel.app/application/faceted-search-modals -->
		<Modal bind:open={filterModalOpen} size="xs" autoclose={false} class="w-full">
			<h3 class="text-xl font-medium text-gray-900 dark:text-white">Filters</h3>
			<div class="grid grid-cols-2 gap-2">
				<div class="col-span-1">
					<h5 class="text-sm dark:text-white py-2">Package State</h5>
					{#each filterPackageStates as fs}
						<Checkbox class="py-1" bind:group={selectedFilterPackageStates} value={fs[1]}>
							{fs[0]}
						</Checkbox>
					{/each}
				</div>

				<div class="col-span-1">
					<h5 class="text-sm dark:text-white py-2">Package Type</h5>
					{#each filterPackageTypes as ft}
						<Checkbox class="py-1" bind:group={selectedFilterPackageTypes} value={ft[1]}>
							{ft[0]}
						</Checkbox>
					{/each}
				</div>
			</div>
			<div class="flex items-center space-x-4 rounded-b dark:border-gray-600" />
		</Modal>
	</div>
</div>
