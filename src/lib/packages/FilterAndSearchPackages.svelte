<script lang="ts">
	import { Badge, Button, Checkbox, Input, Modal, Tooltip } from 'flowbite-svelte';

	import type { Package } from './models';
	import {
		currentPackagesStore,
		filteredPackages,
		packageDiscussionsStore,
		packageLabelsStore,
		searchTermStore
	} from './stores';

	import { IconFilter, IconSearch } from '@tabler/icons-svelte';

	let filterModalOpen = false;

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

	let selectedPackageLabelTypes: string[] = [];

	let selectedFiltersCount = 0;

	$: {
		selectedFiltersCount = 0;

		selectedFiltersCount += selectedFilterPackageStates.length;

		selectedFiltersCount += selectedFilterPackageTypes.length;

		selectedFiltersCount += selectedPackageLabelTypes.length;
	}

	function clearFilters() {
		selectedFilterPackageStates = [];
		selectedFilterPackageTypes = [];
		selectedPackageLabelTypes = [];
	}

	$: {
		let fpkgs: Package[] = $currentPackagesStore;

		if ($searchTermStore) {
			fpkgs = fpkgs.filter(
				(pkg) => pkg.name.toLowerCase().indexOf($searchTermStore.toLowerCase()) !== -1
			);
		}

		if (selectedFilterPackageStates.length > 0) {
			let fstatesFiltered: Package[] = [];
			for (let fs of selectedFilterPackageStates) {
				let fspkgs = fpkgs.filter((pkg) => pkg.state === fs);
				fstatesFiltered.push(...fspkgs);
			}
			fpkgs = fstatesFiltered;
		}

		if (selectedFilterPackageTypes.length > 0) {
			let ftypesFiltered: Package[] = [];
			for (let ft of selectedFilterPackageTypes) {
				let ftpkgs = fpkgs.filter((pkg) => pkg.ptype.toLowerCase() === ft);
				ftypesFiltered.push(...ftpkgs);
			}
			fpkgs = ftypesFiltered;
		}

		if (selectedPackageLabelTypes.length > 0) {
			let flabelsFiltered: Package[] = [];
			for (let ft of selectedPackageLabelTypes) {
				let flpkgs = fpkgs.filter((pkg) =>
					$packageDiscussionsStore[pkg.name]?.labels.find((lb) => lb.name === ft)
				);
				flabelsFiltered.push(...flpkgs);
			}
			fpkgs = flabelsFiltered;
		}

		$filteredPackages = fpkgs;
	}
</script>

<div class="flex items-center gap-x-2">
	<div>
		<div class="relative">
			<Input type="search" class="pl-10" placeholder="Search..." bind:value={$searchTermStore}>
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
		<Modal title="Filters" bind:open={filterModalOpen} size="xs" autoclose={false} class="w-full">
			<Button
				color="alternative"
				size="xs"
				class="text-xs p-1 rounded float-right border-primary-300 dark:border-primary-500 dark:hover:border-primary-700"
				on:click={clearFilters}
			>
				clear all
			</Button>

			<div class="grid grid-cols-2 gap-2">
				<div class="col-span-1">
					<h5 class="text-sm dark:text-white py-2">Package State</h5>
					<hr class="w-2/3 h-px bg-gray-200 rounded border-0 dark:bg-gray-700 mb-3" />

					{#each filterPackageStates as fs}
						<Checkbox class="py-1" bind:group={selectedFilterPackageStates} value={fs[1]}>
							{fs[0]}
						</Checkbox>
					{/each}
				</div>

				<div class="col-span-1">
					<h5 class="text-sm dark:text-white py-2">Package Type</h5>
					<hr class="w-2/3 h-px bg-gray-200 rounded border-0 dark:bg-gray-700 mb-3" />

					{#each filterPackageTypes as ft}
						<Checkbox class="py-1" bind:group={selectedFilterPackageTypes} value={ft[1]}>
							{ft[0]}
						</Checkbox>
					{/each}
				</div>

				<div class="col-span-2">
					<h5 class="text-sm dark:text-white py-2">Label Types</h5>
					<hr class="w-2/3 h-px bg-gray-200 rounded border-0 dark:bg-gray-700 mb-3" />
					<div class="flex flex-wrap flex-row gap-2">
						{#each Object.entries($packageLabelsStore) as [labelName, labelDesc]}
							{@const labelID = `label-cb-${labelName}`}

							<Checkbox
								custom
								class="py-1 col-span-1"
								bind:group={selectedPackageLabelTypes}
								value={labelName}
							>
								<Badge
									rounded
									color="dark"
									id={labelID}
									class="rounded-full border-2 cursor-pointer dark:border-gray-700 peer-checked:border-primary-600 bg-white dark:bg-gray-800 hover:bg-gray-200 dark:hover:bg-gray-700"
									>{labelName}
								</Badge>

								<!-- <Tooltip triggeredBy="#{labelID}" arrow={false} strategy="absolute">
									{labelDesc}
								</Tooltip> -->
							</Checkbox>
						{/each}
					</div>
				</div>
			</div>
			<div class="flex items-center space-x-4 rounded-b dark:border-gray-600" />
		</Modal>
	</div>
</div>
