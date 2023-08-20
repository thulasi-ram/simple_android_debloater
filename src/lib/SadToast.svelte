<script lang="ts">
	import { Toast } from 'flowbite-svelte';
	import { Icon } from 'flowbite-svelte-icons';
	import { fly } from 'svelte/transition';
	import { notifications } from '../notificationStore';
</script>

<!-- <div class="relative h-56"> -->
{#each $notifications as notif(notif.id)}
	{#if notif.type === 'success'}
		<Toast transition={fly} params={{ duration: notif.timeout, x: 200 }} color="green" class="mb-4">
			<Icon name="bell-outline" slot="icon" class="w-5 h-5" />
			{notif.message}
		</Toast>
	{:else if notif.type === 'info'}
		<Toast transition={fly} params={{ duration: notif.timeout, x: 200 }} color="blue" class="mb-4">
			<Icon name="bell-outline" slot="icon" class="w-5 h-5" />
			{notif.message}
		</Toast>
	{:else if notif.type === 'warning'}
		<Toast transition={fly} params={{ duration: notif.timeout, x: 200 }} color="orange" class="mb-4">
			<Icon name="bell-outline" slot="icon" class="w-5 h-5" />
			{notif.message}
		</Toast>
	{:else if notif.type === 'error'}
		<Toast transition={fly} params={{ duration: notif.timeout, x: 200 }} color="red" class="mb-4">
			<Icon name="bell-outline" slot="icon" class="w-5 h-5" />
			{notif.message}
		</Toast>
	{:else}
		<Toast transition={fly} params={{ duration: notif.timeout, x: 200 }} color="gray" class="mb-4">
			<Icon name="bell-outline" slot="icon" class="w-5 h-5" />
			{notif.message}
		</Toast>
	{/if}
{/each}
