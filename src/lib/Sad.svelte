<script>
	import NoDeviceBanner from './NoDeviceBanner.svelte';
	import AdbDeviceWithUsers from './AdbDeviceWithUsers.svelte';
</script>

<div class="space-y-12">
	<NoDeviceBanner refreshButton={AdbDeviceWithUsers} refreshButtonText="Refresh Devices" />
</div>
