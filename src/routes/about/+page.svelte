<script>
	import { IconSlash } from '@tabler/icons-svelte';
	import { Breadcrumb, BreadcrumbItem } from 'flowbite-svelte';
</script>

<Breadcrumb aria-label="Devices BreadCrumb">
	<BreadcrumbItem href="/">
		<svelte:fragment slot="icon">
			<IconSlash size={18} stroke={1.5} />
		</svelte:fragment>
		Home
	</BreadcrumbItem>

	<BreadcrumbItem href="/about">
		<svelte:fragment slot="icon">
			<IconSlash size={18} stroke={1.5} />
		</svelte:fragment>
		About</BreadcrumbItem
	>
</Breadcrumb>
