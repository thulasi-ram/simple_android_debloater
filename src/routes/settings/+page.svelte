<script>
	import { configStore } from '$lib/config/stores';
	import { IconSlash } from '@tabler/icons-svelte';
	import { Breadcrumb, BreadcrumbItem, Checkbox } from 'flowbite-svelte';
</script>

<Breadcrumb aria-label="Devices BreadCrumb">
	<BreadcrumbItem href="/">
		<svelte:fragment slot="icon">
			<IconSlash size={18} stroke={1.5} />
		</svelte:fragment>
		Home
	</BreadcrumbItem>

	<BreadcrumbItem href="/settings">
		<svelte:fragment slot="icon">
			<IconSlash size={18} stroke={1.5} />
		</svelte:fragment>
		Settings</BreadcrumbItem
	>
</Breadcrumb>

<div class="flex gap-y-5 mt-10">
	<Checkbox bind:checked={$configStore.prompt_disable_package} class="cursor:pointer">
		Prompt on Disable Package
	</Checkbox>
</div>
