<script>
	import NoDeviceBanner from './NoDeviceBanner.svelte';
	
	import AdbDeviceWithUsers from './AdbDeviceWithUsers.svelte';
	
	// let trackDevices = '';
	// await listen('rs2js', (event) => {
	// 	console.log('js: rs2js: ' + event);
	// 	let input = event.payload;
	// 	trackDevices = input;
	// });
</script>

<div class="space-y-12">
	<NoDeviceBanner refreshButton={AdbDeviceWithUsers} refreshButtonText="Refresh Devices" />
</div>
