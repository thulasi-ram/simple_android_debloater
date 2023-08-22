<script lang="ts">
	import { page } from '$app/stores';
	import { sadErrorStore } from '$lib/error/stores';
	import { selectedUserIDStore } from '$lib/users/stores';
	import { IconAccessPoint, IconDeviceMobileUp } from '@tabler/icons-svelte';
	import { listen } from '@tauri-apps/api/event';
	import { SidebarItem } from 'flowbite-svelte';
	import { onDestroy, onMount } from 'svelte';
	import type { DeviceWithUsers } from './models';
	import { devicesWithUsersStore, selectedDeviceStore } from './stores';

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

{#if Object.keys($devicesWithUsersStore).length > 0}
	{#each Object.entries($devicesWithUsersStore) as [_, d]}
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
{:else}
	<SidebarItem label="Loading Devices..." class="text-sm text-gray-500 mr-2 border">

		<svelte:fragment slot="subtext">
			<span class="relative flex ml-5">
				<span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-red-400 opacity-75"></span>
				<span class="relative inline-flex rounded-full">
					<IconAccessPoint size={24} stroke={1.5}></IconAccessPoint>
				</span>
			  </span>
		</svelte:fragment>
	</SidebarItem>
{/if}
