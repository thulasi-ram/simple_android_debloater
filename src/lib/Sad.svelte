<script>
	import { Breadcrumb, BreadcrumbItem } from 'flowbite-svelte';
	import NoDeviceBanner from './NoDeviceBanner.svelte';
	import RefreshDevicesButton from './RefreshDevicesButton.svelte';
	import { IconSlash } from '@tabler/icons-svelte';
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

	<NoDeviceBanner refreshButton={RefreshDevicesButton} refreshButtonText="Refresh Devices" />
</div>
