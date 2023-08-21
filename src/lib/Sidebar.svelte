<script>
	import { IconDevicesCode, IconInfoSquareRounded } from '@tabler/icons-svelte';
	import { Sidebar, SidebarGroup, SidebarItem, SidebarWrapper } from 'flowbite-svelte';
	import DevicesSidebarItems from './DevicesSidebarItems.svelte';
</script>

<Sidebar>
	<SidebarWrapper>
		<SidebarGroup>
			<div class="flex gap-x-2 text-lg items-center">
				<IconDevicesCode />
				<h2>Devices</h2>
			</div>
			<DevicesSidebarItems />
		</SidebarGroup>

		<SidebarGroup border>
			<SidebarItem label="About">
				<svelte:fragment slot="icon">
					<IconInfoSquareRounded size={21} />
				</svelte:fragment>
			</SidebarItem>
		</SidebarGroup>
	</SidebarWrapper>
</Sidebar>
