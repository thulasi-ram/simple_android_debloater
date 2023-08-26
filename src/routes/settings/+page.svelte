<script>
	import { configStore } from '$lib/config/stores';
	import { IconSlash } from '@tabler/icons-svelte';
	import { Breadcrumb, BreadcrumbItem, Checkbox, Input, Label } from 'flowbite-svelte';
</script>

<Breadcrumb aria-label="Devices BreadCrumb">
	<BreadcrumbItem href="/">
		<svelte:fragment slot="icon">
			<IconSlash size={18} stroke={1.5} />
		</svelte:fragment>
		Home
	</BreadcrumbItem>

	<BreadcrumbItem>
		<svelte:fragment slot="icon">
			<IconSlash size={18} stroke={1.5} />
		</svelte:fragment>
		Settings</BreadcrumbItem
	>
</Breadcrumb>

<div class="flex flex-col gap-y-5 mt-10">
	{#if $configStore}
		<Checkbox bind:checked={$configStore.prompt_disable_package} class="cursor:pointer">
			Prompt on Disable Package
		</Checkbox>

		<div class="mb-6">
			<Label for="custom-adb-path" class="block mb-2">Custom ADB Path</Label>
			<Input
				id="custom-adb-path"
				placeholder="C:\\Windows\adb\adb.exe (or) ~/Downloads/adb/adb.exe"
				bind:value={$configStore.custom_adb_path}
			/>
		</div>
	{/if}
</div>
