<script lang="ts">
	import {
		Badge,
		Button,
		Table,
		TableBody,
		TableBodyCell,
		TableBodyRow,
		TableHead,
		TableHeadCell
	} from 'flowbite-svelte';

	import { onDestroy, onMount } from 'svelte';
	import type { Unsubscriber } from 'svelte/motion';
	import { filteredPackages } from '../packageStore';

	import {
		disablePackage,
		enablePackage,
		fetchPackagesIfEmptySubscription,
		packageEventListener
	} from './PackagesList';

	let unsub: Unsubscriber = () => {};
	onMount(() => {
		unsub = fetchPackagesIfEmptySubscription();
		packageEventListener();
	});
	onDestroy(unsub);

	let tbCellClass = 'whitespace-nowrap font-small px-2 py-1';
</script>

<div>
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
					</TableBodyCell>
					<TableBodyCell tdClass={tbCellClass}>
						{#if pkg.state == 'Enabled'}
							<Button
								size="xs"
								outline
								color="red"
								class="rounded float-right"
								on:click={() => disablePackage(pkg.name)}>Disable</Button
							>
						{:else if pkg.state == 'Disabled'}
							<Button
								size="xs"
								outline
								color="green"
								class="rounded float-right"
								on:click={() => enablePackage(pkg.name)}>Enable</Button
							>
						{:else}
							<Button size="xs" disabled color="alternative" class="rounded float-right"
								>Unknown State</Button
							>
						{/if}
					</TableBodyCell>
				</TableBodyRow>
			{/each}
		</TableBody>
	</Table>
</div>
