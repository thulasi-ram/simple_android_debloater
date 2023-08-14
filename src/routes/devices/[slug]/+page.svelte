<script>
	import PackagesList from '$lib/PackagesList.svelte';
	import UsersDropdown from '$lib/UsersDropdown.svelte';
	import { adb_list_packages } from '$lib/adb';
	import { Button } from 'flowbite-svelte';
	import { onMount } from 'svelte';
	import { selectedDeviceIDStore } from '../../../stores';

	export let data;

	onMount(() => {
		$selectedDeviceIDStore = data.deviceID;
	});
</script>

<div>
	<div class="flex items-end">
		<UsersDropdown divClass="float-left" />
		<div class="mx-auto"></div>
		<Button class="float-right" on:click={adb_list_packages}>Refresh Packages</Button>
	</div>

	<PackagesList />
</div>
