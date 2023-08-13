<script>
	import NoDeviceBanner from './NoDeviceBanner.svelte';
	import SadError from './SadError.svelte';

	import { onMount } from 'svelte';
	import { adb_list_devices_with_users } from './adb';
	import AdbDeviceWithUsers from './AdbDeviceWithUsers.svelte';


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
	<NoDeviceBanner refreshButton={AdbDeviceWithUsers} />
</div>
