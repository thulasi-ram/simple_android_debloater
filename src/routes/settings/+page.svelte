<script lang="ts">
	import Breadcrumbs from '$lib/BreadCrumbs.svelte';
	import type { Config } from '$lib/config/models';
	import { configStore } from '$lib/config/stores';
	import ErrorModal from '$lib/error/Error.svelte';
	import { notifications } from '$lib/notifications/stores';
	import { sleep } from '$lib/utils';
	import { Button, Checkbox, Input, Label } from 'flowbite-svelte';

	let crumbs = [
		{ name: 'Home', href: '/' },
		{ name: 'Settings', href: '' }
	];

	let config: Config | null = null;
	$: {
		if (config == null && $configStore) {
			config = {
				id: $configStore?.id,
				prompt_disable_package: $configStore?.prompt_disable_package,
				custom_adb_path: $configStore?.custom_adb_path,
				clear_packages_on_disable: $configStore?.clear_packages_on_disable
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
			config = null;
			notifications.info('Config Discarded Successfully!');
			discarding = false;
		});
	}
</script>

<ErrorModal message={updateErr} />

<Breadcrumbs {crumbs} />

<div class="flex flex-col gap-y-5 mt-10">
	{#if config}
		<Checkbox bind:checked={config.prompt_disable_package} class="cursor:pointer">
			Prompt on disable package
		</Checkbox>
		<Checkbox bind:checked={config.clear_packages_on_disable} class="cursor:pointer">
			Clear packages on disable
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
			<Button color="alternative" size="xs" class="dark:hover:border-gray-400" on:click={updateConfig} disabled={saving}>
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
