<script>
	import PackagesList from '$lib/PackagesList.svelte';
	import RefreshPackages from '$lib/RefreshPackages.svelte';
	import UsersDropdown from '$lib/UsersDropdown.svelte';
</script>

<div>
	<div class="flex items-end">
		<UsersDropdown divClass="float-left" />
		<div class="mx-auto" />
		<RefreshPackages />
	</div>

	<PackagesList />
</div>
