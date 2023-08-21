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

	$: activeUrl = $page.url.pathname;


	let activeClass =
	'grid grid-rows-2 grid-cols-none grid-flow-col items-center text-base font-normal text-gray-900 bg-red-200 dark:bg-red-700 rounded-lg dark:text-white hover:bg-red-100 dark:hover:bg-red-700';
	let nonActiveClass =
	'grid grid-rows-2 grid-cols-none grid-flow-col items-center text-base font-normal text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700';
</script>

{#each Object.entries($devicesWithUsersStore) as [ _, d ]}
	{@const dev = d.device}
	{@const hrefUrl = `/devices/${dev.id}`}
	<SidebarItem
		label={dev.name}
		href={hrefUrl}
		active={activeUrl === hrefUrl}
		class="text-sm p-1 grid grid-rows-2 grid-cols-none grid-flow-col items-center border"
		{activeClass}
		{nonActiveClass}
		spanClass="col-span-8"
	>
		<!-- https://tailwindcss.com/docs/grid-row#basic-usage -->
		<svelte:fragment slot="icon">
			<IconDeviceMobileUp class="row-span-2 ml-2" size={24} stroke={1.5} />
		</svelte:fragment>
		<svelte:fragment slot="subtext">
			<span class="col-span-8 text-xxs">({dev.model})</span>
		</svelte:fragment>
	</SidebarItem>
{/each}
