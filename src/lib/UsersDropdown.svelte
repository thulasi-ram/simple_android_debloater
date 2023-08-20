<script lang="ts">
	import {
		Avatar,
		Button,
		Dropdown,
		Label,
		Radio
	} from 'flowbite-svelte';
	import { Icon } from 'flowbite-svelte-icons';
	import { selectedDeviceStore, selectedUserIDStore, selectedUserStore } from '../stores';

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

	{#if $selectedUserStore}
		<Button pill color="light" class="flex p-2 w-48 justify-start justify-items-start">
			<Avatar border class="mr-2">{selectedUserInitials}</Avatar>
			<span class="mr-auto">{$selectedUserStore.name}</span>
			<Icon name="caret-sort-solid" class="w-3 h-3 float-right mr-2 text-gray-500"></Icon>
		</Button>
	{:else}
		<Button pill color="light" class="flex p-2 w-48 justify-start justify-items-start">
			<Avatar border class="mr-2 bg-red-100" classPlaceholder="w-full h-full bg-red-100" >
			</Avatar>
			<span class="mr-auto">Select User</span>
			<Icon name="caret-sort-solid" class="w-3 h-3 float-right mr-2 text-gray-500"></Icon>

		</Button>
	{/if}

	<Dropdown class="w-60 p-3 space-y-1 border rounded">
		{#each userMap as user}
			<li class="rounded-full p-2 hover:bg-gray-100 dark:hover:bg-gray-600 border">
				<Label class="flex items-center gap-x-3 cursor-pointer">
					<Radio
						name="device_user_drop_down"
						bind:group={$selectedUserIDStore}
						value={1}
						class="hidden peer"
					/>
					<Avatar border class="bg-red-100">{user.initials}</Avatar>
					<span>{user.name}</span>
				</Label>
			</li>
		{/each}
	</Dropdown>
</div>
