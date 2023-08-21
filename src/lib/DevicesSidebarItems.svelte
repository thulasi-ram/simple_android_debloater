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
		selectedSidebarItemStore,
		selectedUserIDStore
	} from '../stores';

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

	let activeUrl = '';
	const unsubSelectedSidebarItem = selectedSidebarItemStore.subscribe((val) => {
		activeUrl = val;
	});

	onDestroy(unsubSelectedSidebarItem);

	$: deviceMap = Object.entries($devicesWithUsersStore).map(([_, d]) => ({
		name: `${d.device.name} (${d.device.model})`,
		id: d.device.id
	}));
</script>

{#each deviceMap as { id, name }, i}
	<SidebarItem label={name} href="/devices/{id}" active={activeUrl === '/devices/{id}'}>
		<svelte:fragment slot="icon">
			<IconDeviceMobileUp size={18} />
		</svelte:fragment>
	</SidebarItem>
{/each}
