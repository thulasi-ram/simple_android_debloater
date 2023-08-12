<script>
	import UsersDropdown from '$lib/UsersDropdown.svelte';
	import { onMount } from 'svelte';
	import {
		selectedDeviceIDStore
	} from '../../../stores';
	import PackagesList from '$lib/PackagesList.svelte';

	export let data;

	onMount(() => {
		$selectedDeviceIDStore = data.deviceID;
	});
</script>

<div>
	<UsersDropdown />
	<PackagesList/>
</div>
