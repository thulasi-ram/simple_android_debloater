<script lang="ts">
	import type { Config } from '$lib/config/models';
	import { configStore } from '$lib/config/stores';
	import Error from '$lib/error/Error.svelte';
	import ErrorModal from '$lib/error/Error.svelte';
	import { notifications } from '$lib/notifications/stores';
	import { sleep } from '$lib/utils';
	import { IconSlash } from '@tabler/icons-svelte';
	import {
		Breadcrumb,
		BreadcrumbItem,
		Button,
		Checkbox,
		Helper,
		Input,
		Label
	} from 'flowbite-svelte';

	let config: Config | null = null;
	$: {
		if (config == null && $configStore) {
			config = {
				id: $configStore?.id,
				prompt_disable_package: $configStore?.prompt_disable_package,
				custom_adb_path: $configStore?.custom_adb_path
			};
		}
	}

	let updateErr = '';
	$: saving = false;

	function updateConfig() {
		saving = true;
		sleep(500).then(() => {
			if (config == null) {
				updateErr = 'config is null!!';
			} else {
				configStore.set(config);
			}
			saving = false;
		});
	}

	$: discarding = false;
	function resetConfig() {
		discarding = true;
		sleep(500).then(() => {
			config == null;
			notifications.info('Config Discarded Successfully!');
			discarding = false;
		});
	}
</script>

<ErrorModal message={updateErr} />

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
	{#if config}
		<Checkbox bind:checked={config.prompt_disable_package} class="cursor:pointer">
			Prompt on Disable Package
		</Checkbox>

		<div class="mb-6">
			<Label for="custom-adb-path" class="block mb-2">Custom ADB Path</Label>
			<Input
				id="custom-adb-path"
				placeholder="C:\\Windows\adb\adb.exe (or) ~/Downloads/adb/adb.exe"
				bind:value={config.custom_adb_path}
			/>
		</div>

		<div class="flex gap-x-5">
			<Button color="alternative" size="xs" on:click={updateConfig} disabled={saving}>
				{#if saving}
					Saving...
				{:else}
					Save
				{/if}
			</Button>

			<Button color="red" outline size="xs" on:click={resetConfig} disabled={discarding}>
				{#if discarding}
					Discarding...
				{:else}
					Discard
				{/if}
			</Button>
		</div>
	{/if}
</div>
