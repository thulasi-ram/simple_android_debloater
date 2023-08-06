<script>
	import { Alert } from 'flowbite-svelte';
	import { isPermissionGranted, requestPermission } from '@tauri-apps/api/notification';

	let permissionGranted = false;

	async function onLoad() {
		permissionGranted = await isPermissionGranted();
		if (!permissionGranted) {
			const permission = await requestPermission();
			permissionGranted = permission === 'granted';
		}
	}
</script>

<div class="space-y-12">
	{#if permissionGranted}
		<Alert>"notification permission is missing"</Alert>
	{/if}
</div>
