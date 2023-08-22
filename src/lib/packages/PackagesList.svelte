<script lang="ts">
	import {
		Badge,
		Button,
		Modal,
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow
	} from 'flowbite-svelte';

	import { onDestroy, onMount } from 'svelte';
	import type { Unsubscriber } from 'svelte/motion';
	import { filteredPackages } from './stores';

	import { configStore } from '../config/stores';
	import {
		disablePackage,
		enablePackage,
		fetchPackagesIfEmptySubscription,
		installPackage,
		packageEventListener
	} from './PackagesList';

	let unsub: Unsubscriber = () => {};
	onMount(() => {
		unsub = fetchPackagesIfEmptySubscription();
		packageEventListener();
	});
	onDestroy(unsub);

	let tbCellClass = 'whitespace-nowrap font-small px-2 py-1';

	let disablePackageModal = false;
	let disablePackageName = '';
	function disablePackageButton(pkg: string) {
		if ($configStore.prompt_disable_package) {
			disablePackageName = pkg;
			disablePackageModal = true;
		} else {
			return disablePackage(pkg);
		}
	}
</script>

<div>
	<Modal bind:open={disablePackageModal} size="xs" autoclose>
		<div class="text-center">
			<h3 class="mb-5 text-lg font-normal text-gray-500 dark:text-gray-400">
				Are you sure you want to disable package {disablePackageName}?
			</h3>
			<Button
				color="red"
				class="mr-2"
				on:click={() => {
					disablePackage(disablePackageName);
				}}>Yes, I'm sure</Button
			>
			<Button color="alternative">No, cancel</Button>
		</div>
	</Modal>

	<Table striped={true}>
		<TableBody>
			{#each $filteredPackages as pkg}
				<TableBodyRow>
					<TableBodyCell tdClass={tbCellClass}>
						{pkg.name}
						{#if pkg.ptype == 'system'}
							<Badge rounded color="green">{pkg.ptype}</Badge>
						{:else}
							<Badge rounded color="primary">{pkg.ptype}</Badge>
						{/if}
						<p class="text-xs italic text-gray-500">{pkg.package_prefix}</p>
					</TableBodyCell>
					<TableBodyCell tdClass={tbCellClass}>
						{#if pkg.state == 'Enabled'}
							<Button
								size="xs"
								outline
								color="red"
								class="rounded float-right"
								on:click={() => disablePackageButton(pkg.name)}>Disable</Button
							>
						{:else if pkg.state == 'Disabled'}
							<Button
								size="xs"
								outline
								color="green"
								class="rounded float-right"
								on:click={() => enablePackage(pkg.name)}>Enable</Button
							>
						{:else if pkg.state == 'Uninstalled'}
							<Button
								size="xs"
								outline
								color="green"
								class="rounded float-right"
								on:click={() => installPackage(pkg.name)}>Install</Button
							>
						{:else}
							{@const hideID = `hide-${pkg.name}`}
							<Button disabled id={hideID} size="xs" color="alternative" class="rounded float-right"
								>Hidden</Button
							>
						{/if}
					</TableBodyCell>
				</TableBodyRow>
			{/each}
		</TableBody>
	</Table>
</div>
