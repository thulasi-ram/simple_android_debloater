<script lang="ts">
	import { Toast } from 'flowbite-svelte';
	import { Icon } from 'flowbite-svelte-icons';
	import { fly } from 'svelte/transition';
	import { notifications, type Notif } from './stores';
	import { quintOut } from 'svelte/easing';

	function getColorFromNotif(n: Notif): string {
		// https://stackoverflow.com/a/37978675/6323666
		switch (n.type) {
			case 'success':
				return 'green' as const;
			case 'info':
				return 'blue' as const;
			case 'warning':
				return 'orange' as const;
			case 'error':
				return 'red' as const;
			default:
				return 'gray' as const;
		}
	}

	function getIconFromNotif(n: Notif): string {
		switch (n.type) {
			default:
				return 'bell-outline';
		}
	}
</script>

<!-- <div class="relative h-56"> -->
{#each $notifications as notif (notif.id)}
	{@const ncolor = getColorFromNotif(notif)}
	{@const nicon = getIconFromNotif(notif)}

	<Toast
		transition={fly}
		params={{ duration: notif.timeout, x: 200, easing: quintOut }}
		color={ncolor}
		class="mb-4"
		defaultIconClass="w-4 h-4"
	>
		<Icon name={nicon} slot="icon"/>
		{notif.message}
	</Toast>
{/each}
