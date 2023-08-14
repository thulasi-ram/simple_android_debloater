<script>
	import PackagesList from '$lib/PackagesList.svelte';
	import UsersDropdown from '$lib/UsersDropdown.svelte';
	import { adb_list_packages } from '$lib/adb';
	import { Button } from 'flowbite-svelte';
</script>

<div>
	<div class="flex items-end">
		<UsersDropdown divClass="float-left" />
		<div class="mx-auto" />
		<Button class="float-right" on:click={adb_list_packages}>Refresh Packages</Button>
	</div>

	<PackagesList />
</div>
