<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { Alert, Button, Label, Select } from 'flowbite-svelte';
	import { listen } from '@tauri-apps/api/event';
	import { devicesWithUsersStore, selectedDeviceIDStore, applicableUsersStore } from '../stores';

	let cmdErr = '';
	async function adb_list_devices_with_users() {
		try {
			const cmdOutpt = await invoke('adb_list_devices_with_users');
			devicesWithUsersStore.set(cmdOutpt);
			console.log($devicesWithUsersStore);
			if ($devicesWithUsersStore.length >= 1) {
				$selectedDeviceIDStore = $devicesWithUsersStore[0].device.id;
			}
		} catch (e) {
			cmdErr = String(e);
		}
	}

	async function adb_list_packages() {
		try {
			const cmdOutpt = await invoke('adb_list_packages');
			// devices = cmdOutpt;
		} catch (e) {
			cmdErr = String(e);
		}
	}

	// let trackDevices = '';
	// await listen('rs2js', (event) => {
	// 	console.log('js: rs2js: ' + event);
	// 	let input = event.payload;
	// 	trackDevices = input;
	// });

	$: deviceMap = $devicesWithUsersStore.map((d) => ({
		name: `${d.device.name} (${d.device.model})`,
		value: d.device.id
	}));
</script>

<div class="space-y-12">
	<Alert>
		{cmdErr}
	</Alert>

	<button on:click={adb_list_devices_with_users}>ADB List Devices and Users </button>
	<button on:click={adb_list_packages}>ADB List Packages</button>
	
	<Label
		>Select Device
		<Select class="mt-2" items={deviceMap} bind:value={$selectedDeviceIDStore} />
	</Label>
</div>
