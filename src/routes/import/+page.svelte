<script lang="ts">
	import Breadcrumbs from '$lib/BreadCrumbs.svelte';
	import { sadErrorStore } from '$lib/error/stores';
	import { notifications } from '$lib/notifications/stores';
	import { IconSettingsDown } from '@tabler/icons-svelte';
	import { Button } from 'flowbite-svelte';
	import CSVModal from './csv_parse_modal.svelte';
	import { importSettingsJSON, openAndpParseCSVToJson, type PackageNames } from './import';

	let crumbs = [
		{ name: 'Home', href: '/' },
		{ name: 'Import', href: '' }
	];

	function importSettingsJSONButton() {
		importSettingsJSON()
			.then(() => {
				notifications.success('Imported Settings JSON!');
			})
			.catch((e) => {
				sadErrorStore.setError(e);
			});
	}

	let csvComponent: any;
	let csvComponentProps: {
		title: string;
		open: boolean;
		res: () => Promise<PackageNames> | undefined;
	};

	function bulkDisablePackages() {
		console.log('bulkDisablePackages');
		openAndpParseCSVToJson('Bulk Disable Packages')
			.then((res) => {
				if (!res) {
					return;
				}

				csvComponent = CSVModal;
				csvComponentProps = {
					title: 'Bulk Disable Packages',
					open: true,
					res: res
				};
			})
			.catch((e) => {
				sadErrorStore.setError(e);
			});
	}

	let btnClass =
		'w-64 gap-x-2 font-normal flex flex-row dark:text-gray-300 text-gray-700 hover:border-primary-500 dark:hover:border-primary-500';
	let divClass = 'w-96 flex flex-col items-center';
</script>

<Breadcrumbs {crumbs} />

<svelte:component this={csvComponent} {...csvComponentProps} />

<div class="flex flex-col gap-y-5 mt-10 items-center">
	<div class={divClass}>
		<Button class={btnClass} color="alternative" on:click={importSettingsJSONButton}>
			Import Settings JSON
			<IconSettingsDown stroke={1} />
		</Button>

		<p class="text-xs text-gray-500 text-center">Import settings json previously exported</p>
	</div>

	<div class={divClass}>
		<Button class={btnClass} color="alternative" on:click={bulkDisablePackages}>
			Bulk Disable Packages
			<IconSettingsDown stroke={1} />
		</Button>

		<p class="text-xs text-gray-500 text-center">Bulk Disables Packagess</p>
	</div>
</div>
