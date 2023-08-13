<script>
	import {
		SidebarDropdownItem,
		SidebarDropdownWrapper,
	} from 'flowbite-svelte';
	import { Icon } from 'flowbite-svelte-icons';
	import { onMount } from 'svelte';
	import { devicesWithUsersStore, selectedSidebarItemStore } from '../stores';

	let activeUrl = '';
	onMount(() => {
		selectedSidebarItemStore.subscribe((val) => {
			activeUrl = val;
		});
	});

	$: deviceMap = $devicesWithUsersStore.map((d) => ({
		name: `${d.device.name} (${d.device.model})`,
		id: d.device.id
	}));
</script>

<SidebarDropdownWrapper label="Devices" isOpen={true} active={true}>
	<svelte:fragment slot="icon">
		<Icon
			name="grid-solid"
			class="w-5 h-5 text-gray-500 transition duration-75 dark:text-gray-400 group-hover:text-gray-900 dark:group-hover:text-white"
		/>
	</svelte:fragment>

	{#each deviceMap as { id, name }, i}
		<SidebarDropdownItem label={name} href="/devices/{id}" active={activeUrl === '/devices/{id}'} />
	{/each}
</SidebarDropdownWrapper>
