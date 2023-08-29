<script>
	import Breadcrumbs from '$lib/BreadCrumbs.svelte';
	import { Button } from 'flowbite-svelte';
	import { exportCSV, exportJSON } from './export';
	import { notifications } from '$lib/notifications/stores';
	import { sadErrorStore } from '$lib/error/stores';

	let crumbs = [
		{ name: 'Home', href: '/' },
		{ name: 'Export', href: '' }
	];

	function exportCSVButton() {
		exportCSV()
			.then(() => {
				notifications.success('Exported CSV!');
			})
			.catch((e) => {
				sadErrorStore.setError(e);
			});
	}

	function exportJSONButton() {
		exportJSON()
			.then(() => {
				notifications.success('Exported JSON!');
			})
			.catch((e) => {
				sadErrorStore.setError(e);
			});
	}
</script>

<Breadcrumbs {crumbs} />

<div class="flex flex-col gap-y-5 mt-10 text-gray-700 dark:text-gray-200">
	<Button on:click={exportCSVButton}>Export CSV</Button>
	<Button on:click={exportJSONButton}>Export Advanced JSON</Button>
</div>
