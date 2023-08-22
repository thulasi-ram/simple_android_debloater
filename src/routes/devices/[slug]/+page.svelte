<script>
	import { liveDevicesStore, selectedDeviceStore } from '$lib/devices/stores';
	import FilterAndSearchPackages from '$lib/packages/FilterAndSearchPackages.svelte';
	import PackagesList from '$lib/packages/PackagesList.svelte';
	import RefreshPackagesButton from '$lib/packages/RefreshPackagesButton.svelte';
	import UsersDropdown from '$lib/users/UsersDropdown.svelte';
	import { IconSlash } from '@tabler/icons-svelte';
	import { Breadcrumb, BreadcrumbItem } from 'flowbite-svelte';

	export const deviceId = '';

	let renderPackages = true;

	liveDevicesStore.subscribe((store) => {
		if ($selectedDeviceStore && store[$selectedDeviceStore.device.id]) {
			renderPackages = true;
		} else {
			renderPackages = false;
		}
	});
</script>

<div class="w-full">
	<div class="mb-10">
		<Breadcrumb aria-label="Devices BreadCrumb">
			<BreadcrumbItem href="/">
				<svelte:fragment slot="icon">
					<IconSlash size={18} stroke={1.5} />
				</svelte:fragment>
				Home
			</BreadcrumbItem>
			<BreadcrumbItem>
				<svelte:fragment slot="icon">
					<IconSlash size={18} stroke={1.5} />
				</svelte:fragment>
				Devices</BreadcrumbItem
			>
			<BreadcrumbItem href="/devices/{$selectedDeviceStore?.device.id}">
				<svelte:fragment slot="icon">
					<IconSlash size={18} stroke={1.5} />
				</svelte:fragment>
				{$selectedDeviceStore?.device.name}</BreadcrumbItem
			>
		</Breadcrumb>
	</div>

	<div class="flex items-center gap-x-2 justify-between mb-10">
		<UsersDropdown divClass="relative" />
		<RefreshPackagesButton />
	</div>
	<div class="flex items-baseline gap-x-2 justify-between mb-2">
		<h2 class="text-gray-700 font-semibold">Packages</h2>
		<FilterAndSearchPackages />
	</div>

	{#if renderPackages}
		<PackagesList />
	{/if}
</div>
