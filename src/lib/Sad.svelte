<script>
	import Breadcrumbs from '$lib/BreadCrumbs.svelte';
	import NoDeviceBanner from '$lib/devices/NoDeviceBanner.svelte';
	import RefreshDevicesButton from '$lib/devices/RefreshDevicesButton.svelte';

	let crumbs = [{ name: 'Home', href: '/' }];
</script>

<div class="space-y-12">
	<Breadcrumbs {crumbs} />

	<NoDeviceBanner refreshButton={RefreshDevicesButton} />
</div>
