<script lang="ts">
	import { onMount } from 'svelte';
	import { Button, Modal } from 'flowbite-svelte';
	import { sadErrorStore } from '../stores';

	let errMessage = '';
	let isPermanent = true;

	onMount(() => {
		sadErrorStore.subscribe((val) => {
			errMessage = val.message;
			isPermanent = val.isPermanent;
		});
	});

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
