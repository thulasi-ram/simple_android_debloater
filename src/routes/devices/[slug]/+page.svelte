<script>
	import FilterAndSearchPackages from '$lib/FilterAndSearchPackages.svelte';
	import PackagesList from '$lib/PackagesList.svelte';
	import RefreshPackagesButton from '$lib/RefreshPackagesButton.svelte';
	import UsersDropdown from '$lib/UsersDropdown.svelte';
</script>

<div class="w-3/4">
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
