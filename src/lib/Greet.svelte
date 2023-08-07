<script>
	import { invoke } from '@tauri-apps/api/tauri';
	import { Alert } from 'flowbite-svelte';
	import { listen } from '@tauri-apps/api/event';

	let name = '';
	let greetMsg = '';

	async function greet() {
		console.log('call');
		greetMsg = await invoke('greet', { name });
	}

	let devices = '';
	let devicesErr = '';
	async function adb_list_devices() {
		try {
			const cmdOutpt = await invoke('adb_list_devices');
			devices = cmdOutpt;
		} catch (e) {
			devicesErr = String(e);
		}
	}
	async function adb_list_packages() {
		try {
			const cmdOutpt = await invoke('adb_list_packages');
			devices = cmdOutpt;
		} catch (e) {
			devicesErr = String(e);
		}
	}

	let trackDevices = '';
	await listen('rs2js', (event) => {
		console.log('js: rs2js: ' + event);
		let input = event.payload;
		trackDevices = input;
	});

</script>

<div class="space-y-12">
	<input id="greet-input" placeholder="Enter a name..." bind:value={name} />
	<button on:click={greet}>Greet</button>
	<p>{greetMsg}</p>

	<button on:click={adb_list_devices}>ADB List Devices</button>
	<button on:click={adb_list_packages}>ADB List Packages</button>

	<p>{devices}</p>

	<Alert>
		{devicesErr}
	</Alert>

	<p>trackDevices</p>
	<p>{trackDevices}</p>

</div>
