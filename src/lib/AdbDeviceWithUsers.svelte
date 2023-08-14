<script>
	import { Button } from 'flowbite-svelte';
	import { adb_list_devices_with_users } from './adb';

	export let buttonText = "";
</script>

<Button on:click={adb_list_devices_with_users}>{buttonText}</Button>
