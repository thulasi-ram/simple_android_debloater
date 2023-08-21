<script>
	import { IconDevicesCode, IconInfoSquareRounded } from '@tabler/icons-svelte';
	import { Hr, Sidebar, SidebarGroup, SidebarItem, SidebarWrapper } from 'flowbite-svelte';
	import DevicesSidebarItems from './DevicesSidebarItems.svelte';
</script>

<Sidebar>
	<SidebarWrapper>
		<SidebarGroup>
			<h3 class="text-gray-700 text-xs font-semibold mb-5">DEVICES</h3>
			<DevicesSidebarItems />
		</SidebarGroup>

		<SidebarGroup border>
			<SidebarItem label="About" class="text-sm">
				<svelte:fragment slot="icon">
					<IconInfoSquareRounded size={21} />
				</svelte:fragment>
			</SidebarItem>
		</SidebarGroup>
	</SidebarWrapper>
</Sidebar>
