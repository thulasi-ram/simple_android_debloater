<script>
	import { isPermissionGranted, requestPermission } from '@tauri-apps/api/notification';
	import { Alert } from 'flowbite-svelte';

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
