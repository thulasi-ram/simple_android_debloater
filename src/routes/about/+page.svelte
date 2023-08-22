<script>
	import { IconMessages, IconPackages, IconSlash, IconSourceCode } from '@tabler/icons-svelte';
	import { Breadcrumb, BreadcrumbItem, Button, ButtonGroup, Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell } from 'flowbite-svelte';
	import { getVersion } from '@tauri-apps/api/app';
	const appVersion = getVersion();
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
		About</BreadcrumbItem
	>
</Breadcrumb>

<div class="flex flex-col gap-y-5 mt-10 text-gray-700">
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
			class="gap-x-2 text-gray-700"
			href="https://github.com/thulasi-ram/simple_android_debloater"
			target="_blank"
		>
			<IconSourceCode stroke={1} />
			Source
		</Button>
		<Button
			color="alternative"
			class="gap-x-2 text-gray-700"
			href="https://github.com/thulasi-ram/simple_android_debloater/releases"
			target="_blank"
		>
			<IconPackages stroke={1} />
			Releases
		</Button>
		<Button
			color="alternative"
			class="gap-x-2 text-gray-700"
			href="https://github.com/thulasi-ram/simple_android_debloater/discussions"
			target="_blank"
		>
			<IconMessages stroke={1} />
			Discussions
		</Button>
	</div>


	<p class="text-xl text-gray-700"> Logs </p>
	<ul>
		<li class="pre"><pre> macOS: /Users/Bob/Library/Logs/com.ahiravan.simple-android-debloater</pre> </li>
		<li><pre> Windows:C:\Users\Bob\AppData\Roaming\com.ahiravan.simple-android-debloater\logs </pre></li>
		<li><pre> Linux: /home/bob/.config/com.ahiravan.simple-android-debloater/logs</pre> </li>
	</ul>

	{#await appVersion then version}
		<p class="mx-auto text-gray-500">app version: {version}</p>
	{/await}
</div>
