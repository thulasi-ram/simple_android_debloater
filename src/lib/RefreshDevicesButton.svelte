<script>
	import { Button } from 'flowbite-svelte';
	import { devicesWithUsersStore } from '../deviceUsersStore';
	import { notifications } from '../notificationStore';
	import { adb_list_devices_with_users } from './adb';
	import { setErrorModal } from './utils';

	export let buttonText = '';

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

<Button on:click={list_devices}>{buttonText}</Button>
