<script lang="ts">
	import { page } from '$app/stores';
	import { sadErrorStore } from '$lib/error/stores';
	import { selectedUserIDStore } from '$lib/users/stores';
	import { IconAccessPoint, IconDeviceMobileUp, IconUsb } from '@tabler/icons-svelte';
	import { listen } from '@tauri-apps/api/event';
	import { SidebarItem } from 'flowbite-svelte';
	import { onDestroy, onMount } from 'svelte';
	import type { DeviceWithUsers } from './models';
	import { devicesWithUsersStore, liveDevicesStore, selectedDeviceStore } from './stores';

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

	let defaultClass =
		'text-sm p-1 grid grid-rows-2 grid-cols-none grid-flow-col items-center border';

	let disconnectedClass = defaultClass + " text-gray-300 dark:text-gray-500 dark:border-gray-500"

	let activeClass =
		'grid grid-rows-2 grid-cols-none grid-flow-col items-center text-base font-normal text-gray-900 bg-primary-200 dark:bg-transparent dark:border-primary-500 rounded-lg dark:text-white hover:bg-primary-100 dark:hover:bg-gray-700';
	let nonActiveClass =
		'grid grid-rows-2 grid-cols-none grid-flow-col items-center text-base font-normal text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700';

</script>

{#if Object.keys($devicesWithUsersStore).length > 0}
	{#each Object.entries($devicesWithUsersStore) as [_, d]}
		{@const dev = d.device}
		{@const hrefUrl = `/devices/${dev.id}`}
		{@const isLive = $liveDevicesStore[dev.id]}

		<SidebarItem
			label={dev.name}
			href={isLive ? hrefUrl : "#"}
			active={isLive ? activeUrl === hrefUrl : false}
			class={isLive ? defaultClass : disconnectedClass}
			{activeClass}
			{nonActiveClass}
			spanClass="col-span-8"
		>
			<!-- https://tailwindcss.com/docs/grid-row#basic-usage -->
			<svelte:fragment slot="icon">
				<IconDeviceMobileUp class="row-span-2 ml-2" size={24} stroke={1.5} />
			</svelte:fragment>
			<svelte:fragment slot="subtext">
				<span class="col-span-8 text-xxs">
					({dev.model})
					{#if !isLive}
						Disconnected: {dev.state}
					{/if}
				</span>
			</svelte:fragment>
		</SidebarItem>
	{/each}
{:else}
	<SidebarItem label="Loading Devices..." href="/" class="text-sm text-gray-500 mr-2 border">
		<svelte:fragment slot="subtext">
			<span class="relative flex ml-5">
				<span
					class="animate-ping absolute inline-flex h-full w-full rounded-full bg-primary-400 opacity-75"
				/>
				<span class="relative inline-flex rounded-full">
					<IconUsb size={21} stroke={1.5} />
				</span>
			</span>
		</svelte:fragment>
	</SidebarItem>
{/if}
