<script>
	import Breadcrumbs from '$lib/BreadCrumbs.svelte';
	import { sadErrorStore } from '$lib/error/stores';
	import { notifications } from '$lib/notifications/stores';
	import { DISCUSSIONS_DUMP_URL } from '$lib/packages/discussions';
	import {
		IconArrowUpRight,
		IconCodeAsterix,
		IconSettingsDown,
		IconTableDown
	} from '@tabler/icons-svelte';
	import { Button } from 'flowbite-svelte';
	import { exportAndSaveSettingsJSON, exportPackagesCSV, exportPackagesJSON } from './export';

	let crumbs = [
		{ name: 'Home', href: '/' },
		{ name: 'Export', href: '' }
	];

	function exportPackagesCSVButton() {
		exportPackagesCSV()
			.then(() => {
				notifications.success('Exported Packages CSV!');
			})
			.catch((e) => {
				sadErrorStore.setError(e);
			});
	}

	function exportPackagesJSONButton() {
		exportPackagesJSON()
			.then(() => {
				notifications.success('Exported Devices, Users and Packages JSON!');
			})
			.catch((e) => {
				sadErrorStore.setError(e);
			});
	}

	function exportSettingsJSONButton() {
		exportAndSaveSettingsJSON()
			.then(() => {
				notifications.success('Exported Settings JSON!');
			})
			.catch((e) => {
				sadErrorStore.setError(e);
			});
	}

	let btnClass =
		'w-64 gap-x-2 font-normal dark:hover:border-gray-400 flex flex-row dark:text-gray-200 hover:text-gray-900 hover:border-primary-500 dark:hover:border-primary-500';
	let divClass = 'w-96 flex flex-col items-center';
</script>

<Breadcrumbs {crumbs} />

<div class="flex flex-col gap-y-5 mt-10 items-center">
	<div class={divClass}>
		<Button class={btnClass} color="alternative" on:click={exportPackagesCSVButton}>
			Export CSV
			<IconTableDown stroke={1.5} />
		</Button>
		<p class="text-xs text-gray-500 text-center">Exports packages as a flattened CSV</p>
	</div>

	<div class={divClass}>
		<Button class={btnClass} color="alternative" on:click={exportPackagesJSONButton}>
			Export JSON
			<IconCodeAsterix stroke={1.5} />
		</Button>
		<p class="text-xs text-gray-500 text-center">Exports packages as JSON for advanced usecases</p>
	</div>
	<div class={divClass}>
		<Button class={btnClass} color="alternative" on:click={exportSettingsJSONButton}>
			Export Settings JSON
			<IconSettingsDown stroke={1.5} />
		</Button>
		<p class="text-xs text-gray-500 text-center">Export Settings as JSON</p>
	</div>
	<div class={divClass}>
		<Button class={btnClass} color="alternative" href={DISCUSSIONS_DUMP_URL} target="_blank">
			Package Discussions JSON
			<IconArrowUpRight stroke={1.5} />
		</Button>
		<p class="text-xs text-gray-500 text-center">Open url to discssions_dump.json</p>
	</div>
</div>
