<script>
	import Breadcrumbs from '$lib/BreadCrumbs.svelte';
	import { Button } from 'flowbite-svelte';
	import { exportAndSaveSettingsJSON, exportPackagesCSV, exportPackagesJSON } from './export';
	import { notifications } from '$lib/notifications/stores';
	import { sadErrorStore } from '$lib/error/stores';
	import { DISCUSSIONS_DUMP_URL } from '$lib/packages/discussions';

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
</script>

<Breadcrumbs {crumbs} />

<div class="flex flex-col gap-y-5 mt-10 text-gray-700 dark:text-gray-200">
	<Button on:click={exportPackagesCSVButton}>Export CSV</Button>
	<Button on:click={exportPackagesJSONButton}>Export Advanced JSON</Button>
	<Button on:click={exportSettingsJSONButton}>Export Settings JSON</Button>
	<Button href={DISCUSSIONS_DUMP_URL} target="_blank"> Get Package Discussions JSON</Button>
</div>
