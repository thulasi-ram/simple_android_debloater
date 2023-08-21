<script>
	import Sad from '../lib/Sad.svelte';
	import { selectedDeviceURLStore } from '../stores';

	selectedDeviceURLStore.set("");
</script>

<Sad />

<!-- <style lang="postcss">
	:global(html) {
		background-color: theme(colors.gray.100);
	}
</style> -->
