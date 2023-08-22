<script lang="ts">
	import { Button, Modal } from 'flowbite-svelte';
	import { onDestroy } from 'svelte';
	import { sadErrorStore } from './stores';

	let errMessage = '';
	let isPermanent = true;

	const unsubSadErr = sadErrorStore.subscribe((val) => {
		errMessage = val.message;
		isPermanent = val.isPermanent;
	});

	onDestroy(unsubSadErr);

	function onHide() {
		sadErrorStore.setError('', false);
	}

	$: hasErrMessage = errMessage !== '';
</script>

<div>
	<Modal
		title="Error"
		bind:open={hasErrMessage}
		autoclose={true}
		color="red"
		permanent={isPermanent}
		on:hide={onHide}
	>
		<div class="text-base leading-relaxed">
			{errMessage}
		</div>

		{#if isPermanent}
			<Button href="/">Refresh</Button>
		{/if}
	</Modal>
</div>
