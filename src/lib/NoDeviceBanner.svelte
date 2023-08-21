<script>
	import { IconDevicesSearch } from '@tabler/icons-svelte';
	import { Heading } from 'flowbite-svelte';

	/**
	 * @type {any}
	 */
	export let refreshButton;
	export let refreshButtonProps = {};
</script>

<div class="flex flex-col items-center justify-center">
	<IconDevicesSearch size={96} stroke={1} class="text-gray-300" />
	<p class="text-xl text-gray-400 mt-2 mb-5">No Devices Selected</p>
	<!-- Passing props https://svelte.dev/repl/74593f36569a4c268d8a6ab277db34b5?version=3.12.1 -->
	<svelte:component this={refreshButton} {...refreshButtonProps} />
</div>
