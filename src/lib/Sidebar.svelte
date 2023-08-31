<script>
	import { page } from '$app/stores';
	import DevicesSidebarItems from '$lib/devices/DevicesSidebarItems.svelte';
	import {
		IconDownload,
		IconFileExport,
		IconInfoSquareRounded,
		IconMoon,
		IconMoonFilled,
		IconRefresh,
		IconSettings,
		IconSun,
		IconSunFilled
	} from '@tabler/icons-svelte';
	import {
		Button,
		DarkMode,
		Hr,
		Sidebar,
		SidebarGroup,
		SidebarItem,
		SidebarWrapper
	} from 'flowbite-svelte';
	import { relaunch } from '@tauri-apps/api/process';

	$: activeUrl = $page.url.pathname;

	let darkModeBtnClass =
		'text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg text-xl p-3';
</script>

<Sidebar>
	<SidebarWrapper divClass="bg-white-100">
		<SidebarGroup>
			<h3 class="text-gray-700 text-xs font-semibold mb-5">DEVICES</h3>
			<DevicesSidebarItems />
		</SidebarGroup>

		<SidebarGroup border>
			<SidebarItem
				label="Settings"
				class="text-sm"
				href="/settings"
				active={activeUrl == '/settings'}
			>
				<svelte:fragment slot="icon">
					<IconSettings stroke={1.5} />
				</svelte:fragment>
			</SidebarItem>

			<SidebarItem label="Import" class="text-sm" href="/import" active={activeUrl == '/import'}>
				<svelte:fragment slot="icon">
					<IconDownload stroke={1.5} />
				</svelte:fragment>
			</SidebarItem>

			<SidebarItem label="Export" class="text-sm" href="/export" active={activeUrl == '/export'}>
				<svelte:fragment slot="icon">
					<IconFileExport stroke={1.5} />
				</svelte:fragment>
			</SidebarItem>
		</SidebarGroup>

		<SidebarGroup border>
			<SidebarItem label="About" class="text-sm" href="/about" active={activeUrl == '/about'}>
				<svelte:fragment slot="icon">
					<IconInfoSquareRounded stroke={1.5} />
				</svelte:fragment>
			</SidebarItem>

			<DarkMode class="w-full text-gray-800 dark:text-gray-100">
				<svelte:fragment slot="lightIcon">
					<div class="flex flex-row gap-x-2">
						<IconSun stroke={1.5} size={21} />
						<span class="text-sm">Light Mode</span>
					</div>
				</svelte:fragment>
				<svelte:fragment slot="darkIcon">
					<div class="flex flex-row gap-x-2">
						<IconMoon stroke={1.5} size={21} />
						<span class="text-sm">Dark Mode</span>
					</div>
				</svelte:fragment>
			</DarkMode>

			<SidebarItem
				label="Relaunch app"
				class="text-sm"
				color="alternative"
				on:click={async () => {
					await relaunch();
				}}
			>
				<svelte:fragment slot="icon">
					<IconRefresh stroke={1.5} size={21} />
				</svelte:fragment>
			</SidebarItem>
		</SidebarGroup>
	</SidebarWrapper>
</Sidebar>
