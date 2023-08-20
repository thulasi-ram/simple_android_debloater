<script>
	import NoDeviceBanner from './NoDeviceBanner.svelte';
	import RefreshDevicesButton from './RefreshDevicesButton.svelte';
</script>

<div class="space-y-12">
	<NoDeviceBanner refreshButton={RefreshDevicesButton} refreshButtonText="Refresh Devices" />
</div>
