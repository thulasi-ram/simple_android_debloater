<script>
	import { Breadcrumb, BreadcrumbItem, Checkbox, Heading } from 'flowbite-svelte';
	import { configStore } from '../../stores';
	import { IconSlash } from '@tabler/icons-svelte';
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
