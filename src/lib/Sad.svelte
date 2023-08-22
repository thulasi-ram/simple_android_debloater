<script>
	import NoDeviceBanner from '$lib/devices/NoDeviceBanner.svelte';
	import RefreshDevicesButton from '$lib/devices/RefreshDevicesButton.svelte';
	import { IconSlash } from '@tabler/icons-svelte';
	import { Breadcrumb, BreadcrumbItem } from 'flowbite-svelte';
</script>

<div class="space-y-12">
	<Breadcrumb aria-label="Devices BreadCrumb">
		<BreadcrumbItem href="/">
			<svelte:fragment slot="icon">
				<IconSlash size={18} stroke={1.5} />
			</svelte:fragment>
			Home
		</BreadcrumbItem>
	</Breadcrumb>

	<NoDeviceBanner refreshButton={RefreshDevicesButton} />
</div>
