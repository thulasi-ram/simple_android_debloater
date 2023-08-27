<script lang="ts">
	import { Modal } from 'flowbite-svelte';

	export let message: string;

	$: open = message !== '';
</script>

<div>
	<Modal title="Error" bind:open autoclose={true} color="red">
		<div class="text-base leading-relaxed">
			{message}
		</div>
	</Modal>
</div>
