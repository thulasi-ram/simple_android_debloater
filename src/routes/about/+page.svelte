<script>
	import Breadcrumbs from '$lib/BreadCrumbs.svelte';
	import { IconMessages, IconPackages, IconSourceCode } from '@tabler/icons-svelte';
	import { getVersion } from '@tauri-apps/api/app';
	import { Button } from 'flowbite-svelte';
	import { appLogDir } from '@tauri-apps/api/path';

	let logDir = '';
	appLogDir().then((v) => {
		logDir = v;
	});

	const appVersion = getVersion();

	let crumbs = [
		{ name: 'Home', href: '/' },
		{ name: 'About', href: '' }
	];

	let btnClass =
		'gap-x-2 font-normal dark:hover:border-gray-400 dark:text-gray-200 hover:text-gray-900 hover:border-primary-500 dark:hover:border-primary-500';
</script>

<Breadcrumbs {crumbs} />

<div class="flex flex-col gap-y-5 mt-10 text-gray-700 dark:text-gray-200">
	<p>
		Simpl Android Debloater is a free and open source project to disable unwanted system apps that
		careers / OEMs force install in our mobile phones.
	</p>
	<p>
		This tool is aimed to be beginner friendly so as to not uninstall apps unexpectedly which can
		brick the device.
	</p>

	<div class="space-x-px mx-auto">
		<Button
			color="alternative"
			class={btnClass}
			href="https://github.com/thulasi-ram/simple_android_debloater"
			target="_blank"
		>
			<IconSourceCode stroke={1} />
			Source
		</Button>
		<Button
			color="alternative"
			class={btnClass}
			href="https://github.com/thulasi-ram/simple_android_debloater/releases"
			target="_blank"
		>
			<IconPackages stroke={1} />
			Releases
		</Button>
		<Button
			color="alternative"
			class={btnClass}
			href="https://github.com/thulasi-ram/simple_android_debloater/discussions/categories/packages"
			target="_blank"
		>
			<IconMessages stroke={1} />
			Discussions
		</Button>
	</div>

	<span>
		Logs Directory
		<pre class="text-xs pt-2">{logDir}</pre>
	</span>

	{#await appVersion then version}
		<p class="mx-auto text-gray-500">app version: {version}</p>
	{/await}
</div>
