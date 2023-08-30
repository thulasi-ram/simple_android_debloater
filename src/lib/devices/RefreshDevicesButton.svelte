<script>
	import { setErrorModal } from '$lib/error';
	import { notifications } from '$lib/notifications/stores';
	import { IconRefresh } from '@tabler/icons-svelte';
	import { Button } from 'flowbite-svelte';
	import { adb_list_devices_with_users } from './adb';
	import { devicesWithUsersStore } from './stores';

	function list_devices() {
		notifications.info('fetching devices and users');

		adb_list_devices_with_users()
			.then((devices) => {
				for (let du of devices) {
					devicesWithUsersStore.insertDevice(du);
				}
			})
			.catch(setErrorModal);
	}
</script>

<Button color="alternative" class="gap-x-2 text-gray-500 dark:hover:border-gray-400" on:click={list_devices}>
	Refresh Devices
	<IconRefresh size={21} stroke={1.5} />

</Button>
