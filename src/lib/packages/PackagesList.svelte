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
	import { filteredPackages, packageDiscussionsStore } from './stores';

	import { sadErrorStore } from '$lib/error/stores';
	import { IconChevronDown, IconChevronUp } from '@tabler/icons-svelte';
	import { configStore } from '../config/stores';
	import {
		disablePackage,
		discussion_create_url,
		enablePackage,
		fetchPackagesIfEmptySubscription,
		installPackage,
		packageEventListener
	} from './PackagesList';
	import { loadPackageDiscussions } from './discussions';

	let unsub: Unsubscriber = () => {};
	onMount(() => {
		unsub = fetchPackagesIfEmptySubscription();
		packageEventListener();
		loadPackageDiscussions();
	});
	onDestroy(unsub);

	let tbCellClass = 'whitespace-nowrap font-small px-2 py-1';

	let disablePackageModal = false;
	let disablePackageName = '';
	function disablePackageButton(pkg: string) {
		if (!$configStore) {
			sadErrorStore.setError('config is not loaded yet');
			return;
		}
		if ($configStore.prompt_disable_package) {
			disablePackageName = pkg;
			disablePackageModal = true;
		} else {
			return disablePackage(pkg);
		}
	}

	$: selectedRowPkgName = '';

	function toggleSeletcedRowPkgName(pkg: string) {
		if (selectedRowPkgName !== pkg) {
			selectedRowPkgName = pkg;
		} else {
			selectedRowPkgName = '';
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

	<Table>
		<TableBody>
			{#each $filteredPackages as pkg}
				{@const pkgDiscussion = $packageDiscussionsStore[pkg.name]}
				{@const isDiscussionRowActive = selectedRowPkgName === pkg.name}
				{@const discussionRowActiveClass = isDiscussionRowActive
					? 'border-b-0 bg-gray-100 dark:bg-gray-700'
					: ''}

				<TableBodyRow
					on:click={() => toggleSeletcedRowPkgName(pkg.name)}
					class="cursor-pointer {discussionRowActiveClass}"
				>
					<TableBodyCell tdClass={tbCellClass}>
						{#if !isDiscussionRowActive}
							<button>
								<IconChevronDown stroke={1.5} />
							</button>
						{:else}
							<button><IconChevronUp stroke={1.5} /></button>
						{/if}
					</TableBodyCell>
					<TableBodyCell tdClass={tbCellClass}>
						{pkg.name}
						{#if pkg.ptype == 'system'}
							<Badge rounded color="green">{pkg.ptype}</Badge>
						{:else}
							<Badge rounded color="primary">{pkg.ptype}</Badge>
						{/if}
						<p class="text-xs italic text-gray-500">{pkg.package_prefix}</p>
					</TableBodyCell>

					<TableBodyCell class="py-3" tdClass={tbCellClass}>
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

				{#if isDiscussionRowActive}
					<TableBodyRow>
						<TableBodyCell tdClass={tbCellClass} />
						<TableBodyCell
							colspan="2"
							class="flex flex-col py-3 items-center"
							tdClass={tbCellClass}
						>
							{#if pkgDiscussion}
								<div class="flex gap-3 my-2">
									{#each pkgDiscussion.labels as l}
										<Badge color="dark">{l.name}</Badge>
									{/each}
								</div>

								<span class="text-gray-500 dark:text-gray-400 break-normal whitespace-normal">
									{@html pkgDiscussion.bodyHTML}
								</span>
								<a
									target="_blank"
									href={pkgDiscussion.url}
									class="underline text-gray-500 dark:text-gray-400 text-xs mt-3"
									>View the entire discussion</a
								>
							{:else}
								<p class="text-gray-500 dark:text-gray-400">No Discussion for this package.</p>
								<a
									target="_blank"
									href={discussion_create_url(pkg)}
									class="underline text-gray-500 dark:text-gray-400 text-xs mt-3">Create one?</a
								>
							{/if}
						</TableBodyCell>
					</TableBodyRow>
				{/if}
			{/each}
		</TableBody>
	</Table>
</div>
