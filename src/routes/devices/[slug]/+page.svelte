<script>
	import FilterAndSearchPackages from '$lib/FilterAndSearchPackages.svelte';
	import PackagesList from '$lib/PackagesList.svelte';
	import RefreshPackagesButton from '$lib/RefreshPackagesButton.svelte';
	import UsersDropdown from '$lib/UsersDropdown.svelte';
</script>

<div class="w-3/4">
	<div class="flex items-center gap-x-2">
		<UsersDropdown divClass="float-left" />
		<div class="mx-auto" />
		<FilterAndSearchPackages />
		<RefreshPackagesButton />
	</div>
	<PackagesList />
</div>
