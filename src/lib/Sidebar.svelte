<script>
	import { Sidebar, SidebarGroup, SidebarItem, SidebarWrapper } from 'flowbite-svelte';
	import { Icon } from 'flowbite-svelte-icons';
	import DevicesSidebarDropdown from './DevicesSidebarDropdown.svelte';
</script>

<Sidebar>
	<SidebarWrapper>
		<SidebarGroup>
			<DevicesSidebarDropdown />
		</SidebarGroup>

		<SidebarGroup border>
			<SidebarItem label="About">
				<svelte:fragment slot="icon">
					<Icon
						name="question-circle-solid"
						class="w-5 h-5 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
					/>
				</svelte:fragment>
			</SidebarItem>
		</SidebarGroup>
	</SidebarWrapper>
</Sidebar>
