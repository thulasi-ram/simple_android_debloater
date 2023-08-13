<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { Button } from 'flowbite-svelte';
	import NoDeviceBanner from './NoDeviceBanner.svelte';
	import SadError from './SadError.svelte';

	import { onMount } from 'svelte';
	import { devicesWithUsersStore, sadErrorStore } from '../stores';

	async function adb_list_devices_with_users() {
		sadErrorStore.reset();
		try {
			const cmdOutpt = await invoke('adb_list_devices_with_users');
			devicesWithUsersStore.set(cmdOutpt);
		} catch (e) {
			sadErrorStore.setError(String(e));
		}
	}

	onMount(adb_list_devices_with_users);

	// let trackDevices = '';
	// await listen('rs2js', (event) => {
	// 	console.log('js: rs2js: ' + event);
	// 	let input = event.payload;
	// 	trackDevices = input;
	// });
</script>

<div class="space-y-12">
	<SadError />
	<Button on:click={adb_list_devices_with_users}>ADB List Devices and Users</Button>
	<NoDeviceBanner />
</div>
