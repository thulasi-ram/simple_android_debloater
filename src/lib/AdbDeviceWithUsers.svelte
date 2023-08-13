<script>
	import { Button } from 'flowbite-svelte';
	import { adb_list_devices_with_users } from './adb';
</script>

<Button on:click={adb_list_devices_with_users}>ADB List Devices and Users</Button>
