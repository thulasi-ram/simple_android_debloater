<script>
	import FilterAndSearchPackages from '$lib/FilterAndSearchPackages.svelte';
	import PackagesList from '$lib/PackagesList.svelte';
	import RefreshPackagesButton from '$lib/RefreshPackagesButton.svelte';
	import UsersDropdown from '$lib/UsersDropdown.svelte';
	import { Breadcrumb, BreadcrumbItem } from 'flowbite-svelte';
	import { selectedDeviceStore } from '../../../stores';
	import { IconSlash, IconSlashes } from '@tabler/icons-svelte';

	export const deviceId = '';
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
	<PackagesList />
</div>
