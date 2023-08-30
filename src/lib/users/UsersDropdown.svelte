<script lang="ts">
	import { selectedDeviceStore } from '$lib/devices/stores';
	import { IconSelector } from '@tabler/icons-svelte';
	import { Avatar, Button, Dropdown, Label, Radio } from 'flowbite-svelte';
	import { selectedUserIDStore, selectedUserStore } from './stores';

	export let divClass = '';

	function getInitials(name: string): string {
		return name
			.split(' ')
			.map((n) => n[0])
			.join('');
	}

	$: userMap =
		$selectedDeviceStore?.users.map((u) => ({
			name: u.name,
			value: u.id,
			initials: getInitials(u.name)
		})) || [];

	$: selectedUserInitials = $selectedUserStore?.name ? getInitials($selectedUserStore?.name) : ':/';
</script>

<div class={divClass}>
	<Button pill size="sm" color="light" class="flex p-2 w-48 justify-start justify-items-start">
		{#if $selectedUserStore}
			<Avatar border class="mr-2 bg-primary-100" size="sm" classPlaceholder="w-full h-full bg-primary-100"
				>{selectedUserInitials}</Avatar
			>
			<span class="mr-auto">{$selectedUserStore.name}</span>
		{:else}
			<Avatar
				border
				class="mr-2 bg-primary-100"
				size="sm"
				classPlaceholder="w-full h-full bg-primary-100"
			/>
			<span class="mr-auto">Select User</span>
		{/if}

		<IconSelector stroke={1.5} />
	</Button>

	<Dropdown class="w-60 p-3 space-y-1 border rounded">
		{#each userMap as user}
			<li class="rounded-full p-2 hover:bg-gray-100 dark:hover:bg-gray-600 border">
				<Label class="flex items-center gap-x-3 cursor-pointer">
					<Radio
						name="device_user_drop_down"
						bind:group={$selectedUserIDStore}
						value={user.value}
						class="hidden peer"
					/>
					<Avatar border size="sm" class="bg-primary-100">{user.initials}</Avatar>
					<span>{user.name}</span>
				</Label>
			</li>
		{/each}
	</Dropdown>
</div>
