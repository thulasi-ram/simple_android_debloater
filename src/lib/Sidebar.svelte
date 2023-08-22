<script>
	import { page } from '$app/stores';
	import { IconInfoSquareRounded, IconSettings } from '@tabler/icons-svelte';
	import { Sidebar, SidebarGroup, SidebarItem, SidebarWrapper } from 'flowbite-svelte';
	import DevicesSidebarItems from '$lib/devices/DevicesSidebarItems.svelte';

	$: activeUrl = $page.url.pathname;
</script>

<Sidebar>
	<SidebarWrapper divClass="bg-white-100">
		<SidebarGroup>
			<h3 class="text-gray-700 text-xs font-semibold mb-5">DEVICES</h3>
			<DevicesSidebarItems />
		</SidebarGroup>

		<SidebarGroup border>
			<SidebarItem label="Settings" class="text-sm" href="/settings" active={activeUrl == '/settings'}>
				<svelte:fragment slot="icon">
					<IconSettings stroke={1.5} />
				</svelte:fragment>
			</SidebarItem>
			<SidebarItem label="About" class="text-sm" href="/about" active={activeUrl == '/about'}>
				<svelte:fragment slot="icon">
					<IconInfoSquareRounded stroke={1.5} />
				</svelte:fragment>
			</SidebarItem>
		</SidebarGroup>
	</SidebarWrapper>
</Sidebar>
