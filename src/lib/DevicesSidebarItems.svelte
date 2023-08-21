<script lang="ts">
	import { IconDeviceMobileUp } from '@tabler/icons-svelte';
	import { listen } from '@tauri-apps/api/event';
	import { SidebarItem } from 'flowbite-svelte';
	import { onDestroy, onMount } from 'svelte';
	import { devicesWithUsersStore } from '../deviceUsersStore';
	import type { DeviceWithUsers } from '../models';
	import {
		sadErrorStore,
		selectedDeviceStore,
		selectedUserIDStore
	} from '../stores';
	import { page } from '$app/stores';

	onMount(async () => {
		listen('device_event', (event) => {
			let du = event.payload as DeviceWithUsers;
			devicesWithUsersStore.insertDevice(du);
		});
	});

	const unsubSelectedDeviceStore = selectedDeviceStore.subscribe((sd) => {
		if (sd) {
			if (!sd.users || sd.users.length < 1) {
				return sadErrorStore.setError('UserDetails not found');
			}

			if ($selectedUserIDStore === '') {
				selectedUserIDStore.set(sd.users[0].id);
			}
		}
	});

	onDestroy(unsubSelectedDeviceStore);

	$: deviceMap = Object.entries($devicesWithUsersStore).map(([_, d]) => ({
		name: `${d.device.name} (${d.device.model})`,
		id: d.device.id
	}));

	$: activeUrl = $page.url.pathname;


	let activeClass =
	'flex items-center p-2 text-base font-normal text-gray-900 bg-red-200 dark:bg-red-700 rounded-lg dark:text-white hover:bg-red-100 dark:hover:bg-red-700';
	let nonActiveClass =
	'flex items-center p-2 text-base font-normal text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700';
</script>

{#each deviceMap as { id, name }, i}
	{@const hrefUrl = `/devices/${id}`}
	<SidebarItem
		label={name}
		href={hrefUrl}
		active={activeUrl === hrefUrl}
		class="text-sm"
		{activeClass}
		{nonActiveClass}
	>
		<svelte:fragment slot="icon">
			<IconDeviceMobileUp size={18} />
		</svelte:fragment>
	</SidebarItem>
{/each}
