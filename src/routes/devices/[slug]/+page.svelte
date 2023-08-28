<script>
	import Breadcrumbs from '$lib/BreadCrumbs.svelte';
	import { liveDevicesStore, selectedDeviceStore } from '$lib/devices/stores';
	import FilterAndSearchPackages from '$lib/packages/FilterAndSearchPackages.svelte';
	import PackagesList from '$lib/packages/PackagesList.svelte';
	import RefreshPackagesButton from '$lib/packages/RefreshPackagesButton.svelte';
	import UsersDropdown from '$lib/users/UsersDropdown.svelte';

	export const deviceId = '';

	let renderPackages = true;

	liveDevicesStore.subscribe((store) => {
		if ($selectedDeviceStore && store[$selectedDeviceStore.device.id]) {
			renderPackages = true;
		} else {
			renderPackages = false;
		}
	});

	let crumbs = [
		{ name: 'Home', href: '/' },
		{ name: 'Devices', href: '' },
		{ name: $selectedDeviceStore?.device.name, href: `/devices/${$selectedDeviceStore?.device.id}` }
	];
</script>

<div class="w-full">
	<div class="mb-10">
		<Breadcrumbs {crumbs} />
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
