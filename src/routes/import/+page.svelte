<script lang="ts">
	import Breadcrumbs from '$lib/BreadCrumbs.svelte';
	import { sadErrorStore } from '$lib/error/stores';
	import { notifications } from '$lib/notifications/stores';
	import { CSV_DIALOG_FILTER } from '$lib/utils';
	import { IconSettingsDown } from '@tabler/icons-svelte';
	import { Button, Fileupload, Input, Label } from 'flowbite-svelte';
	import { importSettingsJSON, openDialongSingleFile } from './import';

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

	let bulkActionFile: string | undefined;

	$: {
		console.log('ba', bulkActionFile);
	}

	function bulkDisablePackages() {
		console.log('bulkActionFile', bulkActionFile);
	}

	const inputOpenPathHandler = () => {
		let baFile: string | null = null;
		openDialongSingleFile('Selected Bulk Action CSV', [CSV_DIALOG_FILTER])
			.then((val) => {
				baFile = val;
				bulkActionFile = val ? val : undefined;
				return bulkActionFile;
			})
			.catch((e) => {
				sadErrorStore.setError(e);
			});
		return baFile;
	};

	let btnClass =
		'w-64 gap-x-2 font-normal dark:hover:border-gray-400 flex flex-row dark:text-gray-200 hover:border-primary-500 dark:hover:border-primary-500';
	let divClass = 'w-96 flex flex-col items-center';
</script>

<Breadcrumbs {crumbs} />

<div class="flex flex-col gap-y-5 mt-10 items-center">
	<div class={divClass}>
		<Button class={btnClass} color="alternative" on:click={importSettingsJSONButton}>
			Import Settings JSON
			<IconSettingsDown stroke={1.5} />
		</Button>

		<p class="text-xs text-gray-500 text-center">Import settings json previously exported</p>
	</div>

	<div class={divClass + ' hidden'}>
		<div class="flex flex-col gap-y-5 mt-10 text-gray-700 dark:text-gray-200">
			<Label>
				<Input
					type="file"
					on:click={(e) => {
						e.preventDefault();
						return inputOpenPathHandler();
					}}
					value={bulkActionFile}
				/>
				{bulkActionFile}
			</Label>

			<Fileupload
				on:click={(e) => {
					e.preventDefault();
					return inputOpenPathHandler();
				}}
				bind:x={bulkActionFile}
			/>

			<Button color="alternative" on:click={bulkDisablePackages}>Bulk Disable</Button>
		</div>
	</div>
</div>
