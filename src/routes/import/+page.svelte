<script lang="ts">
	import Breadcrumbs from '$lib/BreadCrumbs.svelte';
	import { sadErrorStore } from '$lib/error/stores';
	import { notifications } from '$lib/notifications/stores';
	import { IconBoxOff, IconCubeSend, IconSettingsDown } from '@tabler/icons-svelte';
	import { Button } from 'flowbite-svelte';
	import CSVModal from '../../lib/packages/PackageCSVEnablerDisabler.svelte';
	import { importSettingsJSON, openAndpParseCSVToJson, type PackageNames } from './import';
	import { adb_disable_package, adb_enable_package } from '$lib/packages/adb';
	import type { Package } from '$lib/packages/models';

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
		processor: (deviceID: string, userID: string, pkg: string) => Promise<Package>;
	};

	function _bulkProcesPackages(
		process_type: string,
		processor: (deviceID: string, userID: string, pkg: string) => Promise<Package>
	) {
		let title = `Bulk ${process_type} packages`;
		openAndpParseCSVToJson(title)
			.then((res) => {
				if (!res) {
					return;
				}

				csvComponent = CSVModal;
				csvComponentProps = {
					title: title,
					open: true,
					res: res,
					processor: processor
				};
			})
			.catch((e) => {
				sadErrorStore.setError(e);
			});
	}

	function bulkDisablePackages() {
		return _bulkProcesPackages('disable', adb_disable_package);
	}

	function bulkEnablePackages() {
		return _bulkProcesPackages('enable', adb_enable_package);
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
			<IconBoxOff stroke={1} />
		</Button>

		<p class="text-xs text-gray-500 text-center">Bulk Disables Packages</p>
	</div>

	<div class={divClass}>
		<Button class={btnClass} color="alternative" on:click={bulkEnablePackages}>
			Bulk Enable Packages
			<IconCubeSend stroke={1} />
		</Button>

		<p class="text-xs text-gray-500 text-center">Bulk Enable Packages</p>
	</div>
</div>
