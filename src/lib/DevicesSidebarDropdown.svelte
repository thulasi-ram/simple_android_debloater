<script>
	import { SidebarDropdownItem, SidebarDropdownWrapper } from 'flowbite-svelte';
	import { Icon } from 'flowbite-svelte-icons';
	import { onDestroy } from 'svelte';
	import {
		devicesWithUsersStore,
		sadErrorStore,
		selectedDeviceStore,
		selectedSidebarItemStore,
		selectedUserIDStore
	} from '../stores';
	import { adb_list_devices_with_users } from './adb';

	let devicesPromise = adb_list_devices_with_users();

	const unsubSelectedDeviceStore = selectedDeviceStore.subscribe((sd) => {
		if (sd) {
			if (!sd.users || sd.users.length < 1) {
				return sadErrorStore.setError('UserDetails not found');
			}

			selectedUserIDStore.set(sd.users[0].id);
		}
	});

	onDestroy(unsubSelectedDeviceStore);

	let activeUrl = '';
	const unsubSelectedSidebarItem = selectedSidebarItemStore.subscribe((val) => {
		activeUrl = val;
	});

	onDestroy(unsubSelectedSidebarItem);

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

	{#await devicesPromise}
		Loading devices...
	{:then devices}
		{#each deviceMap as { id, name }, i}
			<SidebarDropdownItem
				label={name}
				href="/devices/{id}"
				active={activeUrl === '/devices/{id}'}
			/>
		{/each}
	{:catch err}
		{sadErrorStore.setError(err.message)}
	{/await}
</SidebarDropdownWrapper>
